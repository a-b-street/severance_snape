#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::sync::Once;

use geo::{LineString, Point, Polygon};
use geojson::{Feature, GeoJson, Geometry};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

mod osm;
mod parse_osm;
mod scrape;

static START: Once = Once::new();

#[wasm_bindgen]
pub struct MapModel {
    roads: Vec<Road>,
    intersections: Vec<Intersection>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RoadID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct IntersectionID(pub usize);

struct Road {
    id: RoadID,
    way: osm::WayID,
    node1: osm::NodeID,
    node2: osm::NodeID,
    linestring: LineString,
    tags: HashMap<String, String>,
}

struct Intersection {
    id: IntersectionID,
    node: osm::NodeID,
    point: Point,
    roads: Vec<RoadID>,
}

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
            let mut f = Feature::from(Geometry::from(&r.linestring));
            f.set_property("id", r.id.0);
            f.set_property("way", r.way.to_string());
            f.set_property("node1", r.node1.to_string());
            f.set_property("node2", r.node2.to_string());
            for (k, v) in &r.tags {
                f.set_property(k, v.to_string());
            }
            features.push(f);
        }

        let gj = GeoJson::from(features);
        let out = serde_json::to_string(&gj).map_err(err_to_js)?;
        Ok(out)
    }

    /*#[wasm_bindgen(js_name = compareRoute)]
    pub fn compare_route(&self, r: usize) -> Result<String, JsValue> {
        let obj = find_road_width::find_road_width(self, RoadID(r));
        let out = serde_json::to_string(&obj).map_err(err_to_js)?;
        Ok(out)
    }*/
}

#[derive(Deserialize)]
struct CompareRouteRequest {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
