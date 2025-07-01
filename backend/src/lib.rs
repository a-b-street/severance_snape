#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::sync::Once;
use std::time::Duration;

use geo::{Coord, Euclidean, Length, LineString, Point};
use geojson::GeoJson;
use graph::{Graph, RoadID};
use osm_reader::NodeID;
use serde::{Deserialize, Serialize};
use utils::Tags;
use wasm_bindgen::prelude::*;

pub use crate::profiles::Profile;

mod create;
mod disconnected;
mod profiles;
mod route;
mod scores;

static START: Once = Once::new();

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MapModel {
    graph: Graph,
    // Indexed by RoadID
    gradients: Vec<f64>,
    road_kinds: Vec<RoadKind>,
    crossings: Vec<Crossing>,

    // Do we need to update a router's costs?
    walking_settings: Settings,
    cross_anywhere_settings: Settings,
}

#[derive(Serialize, Deserialize)]
struct Crossing {
    osm_id: NodeID,
    point: Coord,
    roads: HashSet<RoadID>,
    tags: Tags,
    kind: CrossingKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CrossingKind {
    Signalized,
    Zebra,
    Other,
}

impl CrossingKind {
    // TODO UK centric and probably wrong...
    pub fn from_tags(tags: &Tags) -> Self {
        if tags.is("crossing", "traffic_signals") {
            return Self::Signalized;
        }
        if tags.is("crossing", "uncontrolled") {
            // TODO And crossing:markings or crossing_ref?
            return Self::Zebra;
        }
        CrossingKind::Other
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RoadKind {
    /// Sidewalks and other pedestrian-oriented
    Footway,
    /// Tagged crossings
    Crossing(CrossingKind),

    /// A big road that can only be crossed at crossings
    Severance,
    /// Other roads that aren't severances and allow pedestrians. If they explicitly have separate
    /// sidewalks tagged, excluded. Otherwise, may or may not actually have sidewalks. Assumed to
    /// be easy enough to walk along and cross anywhere.
    WithTraffic,
}

#[wasm_bindgen]
impl MapModel {
    /// Call either with bytes of an osm.pbf or osm.xml string and a profile name, or a bincoded
    /// file
    #[wasm_bindgen(constructor)]
    pub fn new(is_osm: bool, input_bytes: &[u8], profile: JsValue) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        if is_osm {
            let profile: Profile = serde_wasm_bindgen::from_value(profile)?;
            MapModel::create(input_bytes, profile).map_err(err_to_js)
        } else {
            info!("Deserializing MapModel from {} bytes", input_bytes.len());
            bincode::deserialize_from(input_bytes).map_err(err_to_js)
        }
    }

    /// Returns a GeoJSON string. Just shows the full ped network
    #[wasm_bindgen()]
    pub fn render(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();

        for r in &self.graph.roads {
            let mut f = self.graph.mercator.to_wgs84_gj(&r.linestring);
            f.set_property("kind", format!("{:?}", self.road_kinds[r.id.0]));
            f.set_property("url", r.way.to_string());
            f.set_property("gradient", self.gradients[r.id.0]);
            features.push(f);
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = getCrossings)]
    pub fn get_crossings(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();
        for c in &self.crossings {
            let mut f = self.graph.mercator.to_wgs84_gj(&Point::from(c.point));
            for (k, v) in &c.tags.0 {
                f.set_property("url", c.osm_id.to_string());
                f.set_property("kind", format!("{:?}", c.kind));
                f.set_property(k, v.to_string());
            }
            features.push(f);
        }
        Ok(serde_json::to_string(&GeoJson::from(features)).map_err(err_to_js)?)
    }

    #[wasm_bindgen(js_name = getCrossingDistances)]
    pub fn get_crossing_distances(&self, include_kinds: Vec<String>) -> Result<String, JsValue> {
        Ok(
            scores::get_crossing_distances(self, include_kinds.into_iter().collect())
                .map_err(err_to_js)?,
        )
    }

    #[wasm_bindgen(js_name = compareRoute)]
    pub fn compare_route(&mut self, input: JsValue) -> Result<String, JsValue> {
        let req: CompareRouteRequest = serde_wasm_bindgen::from_value(input)?;
        let start = self.graph.mercator.pt_to_mercator(Coord {
            x: req.x1,
            y: req.y1,
        });
        let end = self.graph.mercator.pt_to_mercator(Coord {
            x: req.x2,
            y: req.y2,
        });
        let (_, gj) = route::do_route(self, start, end, req.settings).map_err(err_to_js)?;
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = scoreDetours)]
    pub fn score_detours(&mut self) -> Result<String, JsValue> {
        let samples = scores::calculate(self, Settings::uk());
        let out = serde_json::to_string(&samples).map_err(err_to_js)?;
        Ok(out)
    }

    /// Return a polygon covering the world, minus a hole for the boundary, in WGS84
    #[wasm_bindgen(js_name = getInvertedBoundary)]
    pub fn get_inverted_boundary(&self) -> Result<String, JsValue> {
        self.graph.get_inverted_boundary().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getBounds)]
    pub fn get_bounds(&self) -> Vec<f64> {
        let b = &self.graph.mercator.wgs84_bounds;
        vec![b.min().x, b.min().y, b.max().x, b.max().y]
    }

    #[wasm_bindgen(js_name = findConnectedComponents)]
    pub fn find_connected_components(&self) -> Result<String, JsValue> {
        let out = serde_json::to_string(&disconnected::find_connected_components(self))
            .map_err(err_to_js)?;
        Ok(out)
    }
}

// Avoid making some fields pub and screwing up wasm_bindgen
impl MapModel {
    pub fn get_graph(&self) -> &Graph {
        &self.graph
    }

    pub fn set_gradients(&mut self, gradients: Vec<f64>) {
        self.gradients = gradients;
    }
}

// Mercator worldspace internally, but not when it comes in from the app
// TODO only use this on the boundary
#[derive(Deserialize)]
pub struct CompareRouteRequest {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    settings: Settings,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    obey_crossings: bool,
    base_speed_mph: f64,
    delay_signalized: f64,
    delay_zebra: f64,
    delay_other: f64,
}

impl Settings {
    pub fn uk() -> Self {
        Self {
            obey_crossings: true,
            base_speed_mph: 3.0,
            delay_signalized: 30.0,
            delay_zebra: 0.0,
            delay_other: 10.0,
        }
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn mph_to_mps(mph: f64) -> f64 {
    mph * 0.44704
}

/// (active time to walk, waiting time)
pub fn cost(
    road_linestring: &LineString,
    kind: RoadKind,
    settings: &Settings,
) -> (Duration, Duration) {
    // TODO Cache the mph_to_mps?
    let speed = mph_to_mps(settings.base_speed_mph);
    let active = Duration::from_secs_f64(Euclidean.length(road_linestring) / speed);
    let waiting = Duration::from_secs_f64(match kind {
        RoadKind::Crossing(CrossingKind::Signalized) => settings.delay_signalized,
        RoadKind::Crossing(CrossingKind::Zebra) => settings.delay_zebra,
        RoadKind::Crossing(CrossingKind::Other) => settings.delay_other,
        _ => 0.0,
    });
    (active, waiting)
}
