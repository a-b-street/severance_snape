use anyhow::Result;
use geo::{Coord, EuclideanLength, LineString};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::{Mode, PathStep};
use serde::Serialize;

use crate::{CompareRouteRequest, MapModel};

// Also returns the line of the snapped request (in WGS84)
pub fn do_route(map: &MapModel, req: CompareRouteRequest) -> Result<(Feature, FeatureCollection)> {
    let start = map.graph.snap_to_road(
        Coord {
            x: req.x1,
            y: req.y1,
        },
        Mode::Foot,
    );
    let end = map.graph.snap_to_road(
        Coord {
            x: req.x2,
            y: req.y2,
        },
        Mode::Foot,
    );

    let route = map.graph.router[Mode::Foot].route(&map.graph, start, end)?;
    let route_linestring = route.linestring(&map.graph);

    let mut directions = Vec::new();
    for step in route.steps {
        if let PathStep::Road { road, .. } = step {
            let r = &map.graph.roads[road.0];
            directions.push(Step {
                name: r.osm_tags.get("name").cloned(),
                way: r.way.to_string(),
                kind: format!("{:?}", map.road_kinds[road.0]),
                layer: r
                    .osm_tags
                    .get("layer")
                    .cloned()
                    .unwrap_or_else(|| "0".to_string()),
            });
        }
    }

    // TODO More exactly, the point snapped to the road?
    let direct_line = LineString::new(vec![
        map.graph.intersections[start.intersection.0].point.into(),
        map.graph.intersections[end.intersection.0].point.into(),
    ]);

    Ok((
        Feature::from(Geometry::from(&map.graph.mercator.to_wgs84(&direct_line))),
        FeatureCollection {
            features: vec![Feature::from(Geometry::from(
                &map.graph.mercator.to_wgs84(&route_linestring),
            ))],
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "direct_length": direct_line.euclidean_length(),
                    "route_length": route_linestring.euclidean_length(),
                    "directions": directions,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        },
    ))
}

#[derive(Serialize)]
struct Step {
    name: Option<String>,
    way: String,
    kind: String,
    layer: String,
}
