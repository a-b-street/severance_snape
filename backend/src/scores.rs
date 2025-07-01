use std::collections::HashSet;

use anyhow::Result;
use geo::{Coord, Densify, Euclidean, Length, Line, LineLocatePoint, LineString, Point};
use geojson::{FeatureCollection, GeoJson};
use graph::RoadID;
use utils::{collapse_degree_2, KeyedLineString, LineSplit};

use crate::{Crossing, MapModel, RoadKind, Settings};

// Walk along severances. Every X meters, try to cross from one side to the other.
//
// We could focus where footways connect to severances, but that's probably a crossing. Ideally we
// want to find footpaths parallel(ish) to severances. If we had some kind of generalized edge
// bundling...
pub fn calculate(map: &mut MapModel, settings: Settings) -> FeatureCollection {
    let mut requests = Vec::new();
    for r in &map.graph.roads {
        if map.road_kinds[r.id.0] == RoadKind::Severance {
            for line in make_perpendicular_offsets(&r.linestring, 25.0, 15.0) {
                requests.push((line.start, line.end));
            }
        }
    }

    let mut samples = Vec::new();
    let mut max_score = 0.0_f64;
    for (start, end) in requests {
        if let Ok((mut f, fc)) = crate::route::do_route(map, start, end, settings) {
            let direct = fc
                .foreign_members
                .as_ref()
                .unwrap()
                .get("direct_length")
                .unwrap()
                .as_f64()
                .unwrap();
            let route = fc
                .foreign_members
                .unwrap()
                .get("route_length")
                .unwrap()
                .as_f64()
                .unwrap();
            let score = route / direct;
            max_score = max_score.max(score);
            f.set_property("score", score);
            samples.push(f);
        }
    }
    info!("Max score is {max_score}");
    FeatureCollection {
        features: samples,
        bbox: None,
        foreign_members: None,
    }
}

// TODO canvas_geometry needs this too
pub fn make_perpendicular_offsets(
    linestring: &LineString,
    walk_every_m: f64,
    project_away_m: f64,
) -> Vec<Line> {
    let mut output = Vec::new();
    // Using lines instead of coords so we can get the angle -- but is this hard to reason about?
    // angle_at_point instead?
    for orig_line in Euclidean.densify(linestring, walk_every_m).lines() {
        // TODO For the last line, use the last point too
        let angle_degs = (orig_line.end.y - orig_line.start.y)
            .atan2(orig_line.end.x - orig_line.start.x)
            .to_degrees();
        let projected_left = project_away(orig_line.start, angle_degs - 90.0, project_away_m);
        let projected_right = project_away(orig_line.start, angle_degs + 90.0, project_away_m);
        output.push(Line::new(projected_left, projected_right));
    }
    output
}

fn project_away(pt: Coord, angle_degs: f64, dist_away_m: f64) -> Coord {
    let (sin, cos) = angle_degs.to_radians().sin_cos();
    Coord {
        x: pt.x + dist_away_m * cos,
        y: pt.y + dist_away_m * sin,
    }
}

pub fn get_crossing_distances(map: &MapModel, include_kinds: HashSet<String>) -> Result<String> {
    // Get all severances, then glue together into a minimal number of lines
    let mut input = Vec::new();
    for road in &map.graph.roads {
        if map.road_kinds[road.id.0] == RoadKind::Severance {
            input.push(KeyedLineString {
                linestring: road.linestring.clone(),
                ids: vec![(road.id, true)],
                key: (),
            });
        }
    }

    let joined_lines = collapse_degree_2(input);
    let split = split_by_crossings(
        joined_lines,
        map.crossings
            .iter()
            .filter(|c| match c.tags.get("crossing") {
                Some(kind) => include_kinds.contains(kind),
                None => include_kinds.contains("unknown"),
            })
            .collect(),
    );

    let mut features = Vec::new();
    for linestring in split {
        let mut f = map.graph.mercator.to_wgs84_gj(&linestring);
        f.set_property("length", Euclidean.length(&linestring));
        features.push(f);
    }
    Ok(serde_json::to_string(&GeoJson::from(features))?)
}

fn split_by_crossings(
    input: Vec<KeyedLineString<RoadID, ()>>,
    crossings: Vec<&Crossing>,
) -> Vec<LineString> {
    let mut output = Vec::new();
    for joined_line in input {
        // Find all crossings on any of the roads belonging to this joined linestring
        let roads: HashSet<RoadID> = joined_line.ids.iter().map(|(r, _)| *r).collect();

        // Even if there are no crossings on this road, never drop any input
        let mut fractions = vec![0.0, 1.0];
        for crossing in &crossings {
            if crossing.roads.is_disjoint(&roads) {
                continue;
            }
            let Some(fraction) = joined_line
                .linestring
                .line_locate_point(&Point::from(crossing.point))
            else {
                continue;
            };
            fractions.push(fraction);
        }

        for ls in joined_line
            .linestring
            .line_split_many(&fractions)
            .unwrap_or_else(Vec::new)
        {
            output.extend(ls);
        }
    }
    output
}
