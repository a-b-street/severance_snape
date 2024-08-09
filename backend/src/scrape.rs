use anyhow::Result;

use crate::{Intersection, IntersectionID, MapModel, Profile, Road, RoadID};

pub fn scrape_osm(input_bytes: &[u8], profile: Profile) -> Result<MapModel> {
    let graph = utils::osm2graph::Graph::new(
        input_bytes,
        |tags| profile.classify(tags).is_some(),
        &mut utils::osm2graph::NullReader,
    )?;

    // Copy all the fields
    let intersections = graph
        .intersections
        .into_iter()
        .map(|i| Intersection {
            id: IntersectionID(i.id.0),
            point: i.point,
            node: i.osm_node,
            roads: i.edges.into_iter().map(|e| RoadID(e.0)).collect(),
        })
        .collect();

    // Add in a bit
    let roads = graph
        .edges
        .into_iter()
        .map(|e| Road {
            id: RoadID(e.id.0),
            src_i: IntersectionID(e.src.0),
            dst_i: IntersectionID(e.dst.0),
            way: e.osm_way,
            node1: e.osm_node1,
            node2: e.osm_node2,
            linestring: e.linestring,
            kind: profile.classify(&e.osm_tags).unwrap(),
            tags: e.osm_tags,
        })
        .collect();

    let (closest_intersection, node_map, ch) = crate::route::build_router(&intersections, &roads);
    let path_calc = fast_paths::create_calculator(&ch);

    Ok(MapModel {
        roads,
        intersections,
        mercator: graph.mercator,
        closest_intersection,
        node_map,
        ch,
        path_calc,
        boundary_polygon: graph.boundary_polygon,
    })
}
