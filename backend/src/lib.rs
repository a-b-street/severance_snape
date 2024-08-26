#[macro_use]
extern crate log;

use std::sync::Once;

use geo::Coord;
use geojson::GeoJson;
use graph::{Direction, Graph, Mode, Road, Timer};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::profiles::Profile;

mod disconnected;
mod fix_osm;
mod heatmap;
mod profiles;
mod route;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct MapModel {
    graph: Graph,
    // Indexed by RoadID. None means the road should be totally ignored from the walking analysis
    road_kinds: Vec<Option<RoadKind>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoadKind {
    Footway,
    Indoors,
    BridgeOrTunnel,
    WithTraffic,
    Crossing,
    Severance,
    // TODO other types of road?
}

#[wasm_bindgen]
impl MapModel {
    /// Call with bytes of an osm.pbf or osm.xml string and a profile name
    #[wasm_bindgen(constructor)]
    pub async fn new(input_bytes: &[u8], profile: JsValue) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        let profile: Profile = serde_wasm_bindgen::from_value(profile)?;

        let mut road_kinds = Vec::new();
        let modify_roads = |roads: &mut Vec<Road>| {
            for r in roads {
                let kind = profile.classify(&r.osm_tags);
                road_kinds.push(kind);
                // Remove some edges from routing
                if kind == None || kind == Some(RoadKind::Severance) {
                    r.access[Mode::Foot] = Direction::None;
                }
            }
        };
        let graph = Graph::new(
            input_bytes,
            graph::GtfsSource::None,
            &mut utils::osm2graph::NullReader,
            modify_roads,
            &mut Timer::new("build graph", None),
        )
        .await
        .map_err(err_to_js)?;

        Ok(MapModel { graph, road_kinds })
    }

    /// Returns a GeoJSON string. Just shows the full ped network
    #[wasm_bindgen()]
    pub fn render(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();

        for r in &self.graph.roads {
            if let Some(kind) = self.road_kinds[r.id.0] {
                let mut f = r.to_gj(&self.graph.mercator);
                f.set_property("kind", format!("{:?}", kind));
                features.push(f);
            }
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = compareRoute)]
    pub fn compare_route(&self, input: JsValue) -> Result<String, JsValue> {
        let req: CompareRouteRequest = serde_wasm_bindgen::from_value(input)?;
        let start = self.graph.mercator.pt_to_mercator(Coord {
            x: req.x1,
            y: req.y1,
        });
        let end = self.graph.mercator.pt_to_mercator(Coord {
            x: req.x2,
            y: req.y2,
        });
        let mode = Mode::parse(&req.mode).map_err(err_to_js)?;
        let (_, gj) = route::do_route(self, start, end, mode).map_err(err_to_js)?;
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = makeHeatmap)]
    pub fn make_heatmap(&self, mode: String) -> Result<String, JsValue> {
        let mode = Mode::parse(&mode).map_err(err_to_js)?;
        // TODO Different strategy for driving
        let samples = heatmap::along_severances(self, mode);
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

    #[wasm_bindgen(js_name = findSeparateSidewalks)]
    pub fn find_separate_sidewalks(&self, duplicates_only: bool) -> Result<String, JsValue> {
        let out = serde_json::to_string(&fix_osm::find_separate_sidewalks(self, duplicates_only))
            .map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = findConnectedComponents)]
    pub fn find_connected_components(&self) -> Result<String, JsValue> {
        let out = serde_json::to_string(&disconnected::find_connected_components(self))
            .map_err(err_to_js)?;
        Ok(out)
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
    mode: String,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
