use geo::{Coord, Densify, Line, LineString};
use geojson::FeatureCollection;

use crate::{MapModel, RoadKind};

// Walk along severances. Every X meters, try to cross from one side to the other.
//
// We could focus where footways connect to severances, but that's probably a crossing. Ideally we
// want to find footpaths parallel(ish) to severances. If we had some kind of generalized edge
// bundling...
pub fn along_severances(map: &MapModel) -> FeatureCollection {
    let mut requests = Vec::new();
    for r in &map.graph.roads {
        if map.road_kinds[r.id.0] == Some(RoadKind::Severance) {
            for line in make_perpendicular_offsets(&r.linestring, 25.0, 15.0) {
                requests.push((line.start, line.end));
            }
        }
    }
    calculate(map, requests)
}

fn calculate(map: &MapModel, requests: Vec<(Coord, Coord)>) -> FeatureCollection {
    let mut samples = Vec::new();
    let mut max_score = 0.0_f64;
    for (start, end) in requests {
        if let Ok((mut f, fc)) = crate::route::do_route(map, start, end) {
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
    for orig_line in linestring.densify(walk_every_m).lines() {
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
