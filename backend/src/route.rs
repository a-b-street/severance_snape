use std::time::Duration;

use anyhow::Result;
use geo::{Coord, Euclidean, Length, LineString};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::{Direction, PathStep};
use itertools::Itertools;
use serde::Serialize;

use crate::{mph_to_mps, MapModel, Settings};

// Also returns the line of the snapped request (in WGS84)
pub fn do_route(
    map: &mut MapModel,
    start: Coord,
    end: Coord,
    settings: Settings,
) -> Result<(Feature, FeatureCollection)> {
    let profile_name = if settings.obey_crossings {
        "walking"
    } else {
        "cross_anywhere"
    };
    let profile = map.graph.profile_names[profile_name];

    // TODO This is getting called upfront after creation; f64 comparisons?
    if (settings.obey_crossings && map.walking_settings != settings)
        || (!settings.obey_crossings && map.cross_anywhere_settings != settings)
    {
        info!("Updating costs for {profile_name}");
        let speed = mph_to_mps(settings.base_speed_mph);

        for road in &mut map.graph.roads {
            if road.access[profile.0] == Direction::Both {
                road.cost[profile.0] =
                    Duration::from_secs_f64(Euclidean.length(&road.linestring) / speed);
            }
        }
        map.graph.routers[profile.0].update_costs(&map.graph.roads, profile);
        if settings.obey_crossings {
            map.walking_settings = settings;
        } else {
            map.cross_anywhere_settings = settings;
        }
    }

    let start = map.graph.snap_to_road(start, profile);
    let end = map.graph.snap_to_road(end, profile);

    let route = map.graph.routers[profile.0].route(&map.graph, start, end)?;
    let route_linestring = route.linestring(&map.graph);

    let mut duration = Duration::ZERO;
    let mut directions = Vec::new();
    for (pos, step) in route.steps.into_iter().with_position() {
        if let PathStep::Road { road, forwards } = step {
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

            let percent = match pos {
                itertools::Position::First => {
                    if forwards {
                        1.0 - route.start.fraction_along
                    } else {
                        route.start.fraction_along
                    }
                }
                itertools::Position::Last => {
                    if forwards {
                        route.end.fraction_along
                    } else {
                        1.0 - route.end.fraction_along
                    }
                }
                itertools::Position::Middle => 1.0,
                itertools::Position::Only => {
                    (route.end.fraction_along - route.start.fraction_along).abs()
                }
            };
            duration += r.cost[profile.0].mul_f64(percent);
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
                    "direct_length": Euclidean.length(&direct_line),
                    "route_length": Euclidean.length(&route_linestring),
                    "directions": directions,
                    "duration_s": duration.as_secs(),
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
