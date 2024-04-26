use anyhow::Result;
use utils::Tags;

use crate::{Intersection, IntersectionID, MapModel, Road, RoadID, RoadKind};

pub fn scrape_osm(
    input_bytes: &[u8],
    import_streets_without_sidewalk_tagging: bool,
) -> Result<MapModel> {
    let graph = utils::osm2graph::Graph::new(input_bytes, |tags| {
        classify(tags, import_streets_without_sidewalk_tagging).is_some()
    })?;

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
            kind: classify(&e.osm_tags, import_streets_without_sidewalk_tagging).unwrap(),
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

/// This function classifies an OSM way as a RoadKind. If it returns `None`, then the way is
/// totally excluded from the walking graph.
// TODO This should probably be configurable per region. In Hong Kong, primary and above are
// severances. In some places, maybe secondary or tertiary should also be considered severances.
fn classify(tags: &Tags, import_streets_without_sidewalk_tagging: bool) -> Option<RoadKind> {
    if !tags.has("highway") || tags.is("highway", "proposed") || tags.is("area", "yes") {
        return None;
    }

    // Some kind of explicit footway
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

    // Big roads are always severances.
    // TODO Big roads without separate sidewalks aren't walkable at all right now.
    // https://github.com/dabreegster/severance_snape/issues/5
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

    // Totally exclude roads that claim to have a separately mapped sidewalk; they're just noise.
    // I'm assuming there isn't a silly mix like "sidewalk:left = separate, sidewalk:right = yes".
    if tags.is("sidewalk", "separate")
        || tags.is("sidewalk:left", "separate")
        || tags.is("sidewalk:right", "separate")
        || tags.is("sidewalk:both", "separate")
    {
        return None;
    }

    if tags.is("highway", "pedestrian") || tags.is_any("sidewalk", vec!["both", "right", "left"]) {
        return Some(RoadKind::WithTraffic);
    }

    // No sidewalk tagging. We can make a guess about which ones are still routable for walking. In
    // places with thoroughly tagged sidewalks, disable this. Keeping this on is usually messy,
    // because there'll be a mix of separately mapped RoadKind::Footways and then one of these
    // RoadKind::WithTraffic in the middle.
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
        if import_streets_without_sidewalk_tagging {
            return Some(RoadKind::WithTraffic);
        } else {
            return None;
        }
    }

    // TODO highway=construction?

    // TODO Maybe just use tagged / assumed speed limit instead of highway classification?

    // TODO wait, why's this the fallback case?
    Some(RoadKind::Severance)
}
