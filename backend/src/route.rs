use anyhow::{bail, Result};
use fast_paths::{FastGraph, InputGraph};
use geo::HaversineLength;
use geojson::GeoJson;
use rstar::primitives::GeomWithData;
use rstar::RTree;

use crate::node_map::NodeMap;
use crate::{
    CompareRouteRequest, Intersection, IntersectionID, IntersectionLocation, MapModel, Road,
};

pub fn build_router(
    intersections: &Vec<Intersection>,
    roads: &Vec<Road>,
) -> (
    RTree<IntersectionLocation>,
    NodeMap<IntersectionID>,
    FastGraph,
) {
    let mut input_graph = InputGraph::new();
    let mut node_map = NodeMap::new();

    for r in roads {
        let node1 = node_map.get_or_insert(r.src_i);
        let node2 = node_map.get_or_insert(r.dst_i);
        let cost = r.linestring.haversine_length() as usize;
        input_graph.add_edge(node1, node2, cost);
        input_graph.add_edge(node2, node1, cost);
    }
    input_graph.freeze();
    let ch = fast_paths::prepare(&input_graph);

    let closest_intersection = build_closest_intersection(intersections, &node_map);
    (closest_intersection, node_map, ch)
}

// TODO We may be able to override the distance function? Does it work with WGS84?
fn build_closest_intersection(
    intersections: &Vec<Intersection>,
    node_map: &NodeMap<IntersectionID>,
) -> RTree<IntersectionLocation> {
    let mut points = Vec::new();
    for i in intersections {
        points.push(IntersectionLocation::new(
            i.point.into(),
            node_map.get(i.id),
        ));
    }
    RTree::bulk_load(points)
}

pub fn do_route(map: &mut MapModel, req: CompareRouteRequest) -> Result<GeoJson> {
    let start = map
        .closest_intersection
        .nearest_neighbor(&[req.x1, req.y1])
        .unwrap()
        .data;
    let end = map
        .closest_intersection
        .nearest_neighbor(&[req.x2, req.y2])
        .unwrap()
        .data;
    if start == end {
        bail!("start = end");
    }

    if let Some(path) = map.path_calc.calc_path(&map.ch, start, end) {
        let mut features = Vec::new();
        for pair in path.get_nodes().windows(2) {
            let src_i = map.node_map.translate_id(pair[0]);
            let dst_i = map.node_map.translate_id(pair[1]);
            let road = map.find_edge(src_i, dst_i);
            features.push(road.to_gj());
        }
        return Ok(GeoJson::from(features));
    }
    bail!("No path");
}
