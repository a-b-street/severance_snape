use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, Geometry, GeometryCollection, LineString, MapCoordsInPlace, Point};
use osm_reader::{Element, NodeID, WayID};

use crate::mercator::Mercator;
use crate::tags::Tags;
use crate::{Intersection, IntersectionID, MapModel, Road, RoadID, RoadKind};

struct Way {
    id: WayID,
    node_ids: Vec<NodeID>,
    tags: Tags,
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<MapModel> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    for elem in osm_reader::parse(input_bytes)? {
        match elem {
            Element::Node { id, lon, lat, .. } => {
                let pt = Coord { x: lon, y: lat };
                node_mapping.insert(id, pt);
            }
            Element::Way { id, node_ids, tags } => {
                if tags.contains_key("highway") {
                    highways.push(Way {
                        id,
                        node_ids,
                        tags: tags.into(),
                    });
                }
            }
            Element::Relation { .. } => {}
        }
    }

    let (mut roads, mut intersections) = split_edges(&node_mapping, highways);

    // TODO expensive
    let collection: GeometryCollection = roads
        .iter()
        .map(|r| Geometry::LineString(r.linestring.clone()))
        .chain(
            intersections
                .iter()
                .map(|i| Geometry::Point(i.point.clone())),
        )
        .collect::<Vec<_>>()
        .into();
    let mercator = Mercator::from(collection).unwrap();
    for r in &mut roads {
        r.linestring
            .map_coords_in_place(|pt| mercator.to_mercator(pt));
    }
    for i in &mut intersections {
        i.point.map_coords_in_place(|pt| mercator.to_mercator(pt));
    }

    let (closest_intersection, node_map, ch) = crate::route::build_router(&intersections, &roads);
    let path_calc = fast_paths::create_calculator(&ch);

    Ok(MapModel {
        roads,
        intersections,
        mercator,
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
    let mut node_to_intersection: HashMap<NodeID, IntersectionID> = HashMap::new();
    let mut intersections = Vec::new();
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
                    let intersection = if let Some(i) = node_to_intersection.get(&n) {
                        &mut intersections[i.0]
                    } else {
                        let i = IntersectionID(intersections.len());
                        intersections.push(Intersection {
                            id: i,
                            node: n,
                            point: Point(point),
                            roads: Vec::new(),
                        });
                        node_to_intersection.insert(n, i);
                        &mut intersections[i.0]
                    };

                    intersection.roads.push(road_id);
                    i_ids.push(intersection.id);
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
                    kind: classify(&way.tags),
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    (roads, intersections)
}

fn classify(tags: &Tags) -> RoadKind {
    if tags.is("highway", "footway") {
        // TODO These aren't mutually exclusive...
        if tags.has("indoor") {
            return RoadKind::Indoors;
        }
        if tags.has_any(vec!["layer", "bridge", "tunnel"]) {
            return RoadKind::BridgeOrTunnel;
        }
        if tags.is("footway", "crossing") {
            return RoadKind::Crossing;
        }
        return RoadKind::Footway;
    }

    if tags.is("highway", "crossing") || tags.has("crossing") {
        return RoadKind::Crossing;
    }

    if tags.is("highway", "pedestrian") || tags.is_any("sidewalk", vec!["both", "right", "left"]) {
        return RoadKind::Sidewalk;
    }
    // If sidewalks aren't tagged, still assume most streets have them
    // Exclude primary from this list for HK cases
    // TODO But this makes things much messier; sidewalk=separate is not tagged often, but we
    // should infer it
    if tags.is_any("highway", vec!["secondary", "tertiary", "residential"])
        && !tags.is("foot", "no")
        && !tags.is_any("sidewalk", vec!["no", "none", "separate"])
    {
        return RoadKind::Sidewalk;
    }

    RoadKind::Severance
}
