use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, MapCoordsInPlace, Point, Polygon};

use crate::osm::{NodeID, OsmID, WayID};
use crate::parse_osm::Element;
use crate::{Intersection, IntersectionID, MapModel, Road, RoadID};

struct Way {
    id: WayID,
    node_ids: Vec<NodeID>,
    tags: HashMap<String, String>,
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<MapModel> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    for elem in crate::parse_osm::parse_osm(input_bytes)? {
        match elem {
            Element::Node { id, pt, .. } => {
                node_mapping.insert(id, pt);
            }
            Element::Way { id, node_ids, tags } => {
                if is_any(&tags, "highway", vec!["footway", "pedestrian"]) {
                    highways.push(Way { id, node_ids, tags });
                } else if tags.contains_key("highway")
                    && is_any(&tags, "sidewalk", vec!["both", "right", "left"])
                {
                    highways.push(Way { id, node_ids, tags });
                }
            }
            Element::Relation { .. } => {}
        }
    }

    let (roads, intersections) = split_edges(&node_mapping, highways);
    let (closest_intersection, node_map, ch) = crate::route::build_router(&intersections, &roads);
    let path_calc = fast_paths::create_calculator(&ch);

    Ok(MapModel {
        roads,
        intersections,
        closest_intersection,
        node_map,
        ch,
        path_calc,
    })
}

fn split_edges(
    node_mapping: &HashMap<NodeID, Coord>,
    ways: Vec<Way>,
) -> (Vec<Road>, Vec<Intersection>) {
    // Count how many ways reference each node
    let mut node_counter: HashMap<NodeID, usize> = HashMap::new();
    for way in &ways {
        for node in &way.node_ids {
            *node_counter.entry(*node).or_insert(0) += 1;
        }
    }

    // Split each way into edges
    let mut intersections = BTreeMap::new();
    let mut roads = Vec::new();
    for way in ways {
        let mut node1 = way.node_ids[0];
        let mut pts = Vec::new();

        let num_nodes = way.node_ids.len();
        for (idx, node) in way.node_ids.into_iter().enumerate() {
            pts.push(node_mapping[&node]);
            // Edges start/end at intersections between two ways. The endpoints of the way also
            // count as intersections.
            let is_endpoint =
                idx == 0 || idx == num_nodes - 1 || *node_counter.get(&node).unwrap() > 1;
            if is_endpoint && pts.len() > 1 {
                let road_id = RoadID(roads.len());

                let mut i_ids = Vec::new();
                for (n, point) in [(node1, pts[0]), (node, *pts.last().unwrap())] {
                    let next_id = IntersectionID(intersections.len());
                    let i = intersections.entry(n).or_insert_with(|| Intersection {
                        id: next_id,
                        node: n,
                        point: Point(point),
                        roads: Vec::new(),
                    });
                    i.roads.push(road_id);
                    i_ids.push(i.id);
                }

                roads.push(Road {
                    id: road_id,
                    src_i: i_ids[0],
                    dst_i: i_ids[1],
                    way: way.id,
                    node1,
                    node2: node,
                    linestring: LineString::new(std::mem::take(&mut pts)),
                    tags: way.tags.clone(),
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    let intersections = intersections.into_values().collect();

    (roads, intersections)
}

fn is_any(tags: &HashMap<String, String>, k: &str, values: Vec<&str>) -> bool {
    if let Some(v) = tags.get(k) {
        values.contains(&v.as_ref())
    } else {
        false
    }
}
