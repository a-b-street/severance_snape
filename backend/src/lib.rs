#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::fmt;
use std::sync::Once;

use fast_paths::{FastGraph, PathCalculator};
use geo::{LineString, Point};
use geojson::{Feature, GeoJson, Geometry};
use rstar::{primitives::GeomWithData, RTree};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod node_map;
mod osm;
mod parse_osm;
mod route;
mod scrape;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct MapModel {
    roads: Vec<Road>,
    intersections: Vec<Intersection>,
    closest_intersection: RTree<IntersectionLocation>,
    node_map: node_map::NodeMap<IntersectionID>,
    ch: FastGraph,
    path_calc: PathCalculator,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RoadID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct IntersectionID(pub usize);

impl fmt::Display for RoadID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Road #{}", self.0)
    }
}

impl fmt::Display for IntersectionID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intersection #{}", self.0)
    }
}

pub struct Road {
    id: RoadID,
    src_i: IntersectionID,
    dst_i: IntersectionID,
    way: osm::WayID,
    node1: osm::NodeID,
    node2: osm::NodeID,
    linestring: LineString,
    tags: HashMap<String, String>,
}

pub struct Intersection {
    id: IntersectionID,
    #[allow(dead_code)]
    node: osm::NodeID,
    point: Point,
    roads: Vec<RoadID>,
}

// fast_paths ID representing the OSM node ID as the data
type IntersectionLocation = GeomWithData<[f64; 2], usize>;

#[wasm_bindgen]
impl MapModel {
    /// Call with bytes of an osm.pbf or osm.xml string
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        scrape::scrape_osm(input_bytes).map_err(err_to_js)
    }

    /// Returns a GeoJSON string. Just shows the full ped network
    #[wasm_bindgen()]
    pub fn render(&mut self) -> Result<String, JsValue> {
        let mut features = Vec::new();

        for r in &self.roads {
            features.push(r.to_gj());
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = compareRoute)]
    pub fn compare_route(&mut self, input: JsValue) -> Result<String, JsValue> {
        let req: CompareRouteRequest = serde_wasm_bindgen::from_value(input)?;
        let gj = route::do_route(self, req).map_err(err_to_js)?;
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    fn find_edge(&self, i1: IntersectionID, i2: IntersectionID) -> &Road {
        // TODO Store lookup table
        for r in &self.intersections[i1.0].roads {
            let road = &self.roads[r.0];
            if road.src_i == i2 || road.dst_i == i2 {
                return road;
            }
        }
        panic!("no road from {i1} to {i2} or vice versa");
    }
}

impl Road {
    fn to_gj(&self) -> Feature {
        let mut f = Feature::from(Geometry::from(&self.linestring));
        f.set_property("id", self.id.0);
        f.set_property("way", self.way.to_string());
        f.set_property("node1", self.node1.to_string());
        f.set_property("node2", self.node2.to_string());
        for (k, v) in &self.tags {
            f.set_property(k, v.to_string());
        }
        f
    }
}

#[derive(Deserialize)]
pub struct CompareRouteRequest {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
