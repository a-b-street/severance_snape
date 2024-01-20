use std::collections::HashMap;

use anyhow::Result;
use geo::{ConvexHull, Coord, Geometry, GeometryCollection, LineString, Point};
use osm_reader::{Element, NodeID, WayID};

use crate::common::{Mercator, Tags};
use crate::{Intersection, IntersectionID, MapModel, Road, RoadID, RoadKind};

struct Way {
    id: WayID,
    node_ids: Vec<NodeID>,
    tags: Tags,
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<MapModel> {
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    osm_reader::parse(input_bytes, |elem| match elem {
        Element::Node { id, lon, lat, .. } => {
            let pt = Coord { x: lon, y: lat };
            node_mapping.insert(id, pt);
        }
        Element::Way { id, node_ids, tags } => {
            let tags: Tags = tags.into();
            if classify(&tags).is_some() {
                highways.push(Way { id, node_ids, tags });
            }
        }
        Element::Relation { .. } => {}
    })?;

    let (mut roads, mut intersections) = split_edges(&node_mapping, highways);

    // TODO expensive
    let mut collection: GeometryCollection = roads
        .iter()
        .map(|r| Geometry::LineString(r.linestring.clone()))
        .chain(
            intersections
                .iter()
                .map(|i| Geometry::Point(i.point.clone())),
        )
        .collect::<Vec<_>>()
        .into();
    let mercator = Mercator::from(collection.clone()).unwrap();
    for r in &mut roads {
        mercator.to_mercator_in_place(&mut r.linestring);
    }
    for i in &mut intersections {
        mercator.to_mercator_in_place(&mut i.point);
    }

    mercator.to_mercator_in_place(&mut collection);
    let boundary_polygon = collection.convex_hull();

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
        boundary_polygon,
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
                    kind: classify(&way.tags).unwrap(),
                });

                // Start the next edge
                node1 = node;
                pts.push(node_mapping[&node]);
            }
        }
    }

    (roads, intersections)
}

// TODO This should probably be configurable per region. In HK, primary and above are severances.
// And we need to handle roads with sidewalk tags; we can't just assume footways everywhere.
fn classify(tags: &Tags) -> Option<RoadKind> {
    if !tags.has("highway") || tags.is("highway", "proposed") || tags.is("area", "yes") {
        return None;
    }

    if tags.is_any(
        "highway",
        vec!["footway", "steps", "path", "track", "corridor"],
    ) {
        // TODO These aren't mutually exclusive...
        if tags.has("indoor") || tags.is("highway", "corridor") {
            return Some(RoadKind::Indoors);
        }
        if tags.has_any(vec!["layer", "bridge", "tunnel"]) {
            return Some(RoadKind::BridgeOrTunnel);
        }
        if tags.is("footway", "crossing") {
            return Some(RoadKind::Crossing);
        }
        return Some(RoadKind::Footway);
    }

    if tags.is("highway", "crossing") || tags.has("crossing") {
        return Some(RoadKind::Crossing);
    }

    // Even if a big road has a sidewalk, it's a severance
    // TODO Ideally these'd have separate sidewalks
    if tags.is_any(
        "highway",
        vec![
            "motorway",
            "motorway_link",
            "trunk",
            "trunk_link",
            "primary",
            "primary_link",
        ],
    ) {
        return Some(RoadKind::Severance);
    }

    // Totally exclude these; they're just noise. We'll use the separate footways instead.
    // I'm assuming there isn't a silly mix like "sidewalk:left = separate, sidewalk:right = yes".
    if tags.is("sidewalk", "separate")
        || tags.is("sidewalk:left", "separate")
        || tags.is("sidewalk:right", "separate")
    {
        return None;
    }

    if tags.is("highway", "pedestrian") || tags.is_any("sidewalk", vec!["both", "right", "left"]) {
        return Some(RoadKind::WithTraffic);
    }
    // If sidewalks aren't tagged, still assume most classes of streets have them
    // TODO If these happen to have a separate sidewalk but sidewalk=separate is not tagged, then
    // this will make things messier.
    if tags.is_any(
        "highway",
        vec![
            "secondary",
            "secondary_link",
            "tertiary",
            "tertiary_link",
            "residential",
            "unclassified",
            "service",
            "living_street",
            "cycleway",
        ],
    ) && !tags.is("foot", "no")
    {
        return Some(RoadKind::WithTraffic);
    }

    // TODO construction?

    // TODO Maybe just use tagged / assumed speed limit instead?

    // TODO wait, why's this the fallback case?
    Some(RoadKind::Severance)
}
