use geo::{
    BoundingRect, DensifyHaversine, Geometry, GeometryCollection, HaversineBearing,
    HaversineDestination, Line, LineString, Point, Rect,
};
use geojson::{Feature, FeatureCollection};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{CompareRouteRequest, MapModel, RoadKind};

pub fn measure_randomly(map: &mut MapModel, n: usize) -> FeatureCollection {
    // TODO Expensive
    let bbox: Rect<f64> = map
        .roads
        .iter()
        .map(|r| Geometry::LineString(r.linestring.clone()))
        .collect::<GeometryCollection>()
        .bounding_rect()
        .unwrap();
    // TODO Do this in the right coordinate space
    let dist_away = 0.01;

    let mut rng = XorShiftRng::seed_from_u64(42);
    let mut requests = Vec::new();
    for _ in 0..n {
        let x1 = rng.gen_range(bbox.min().x..=bbox.max().x);
        let y1 = rng.gen_range(bbox.min().y..=bbox.max().y);
        let x2 = x1 + rng.gen_range(-dist_away..=dist_away);
        let y2 = y1 + rng.gen_range(-dist_away..=dist_away);
        requests.push(CompareRouteRequest { x1, y1, x2, y2 });
    }
    calculate(map, requests)
}

// Walk along severances. Every X meters, try to cross from one side to the other.
//
// We could focus where footways connect to severances, but that's probably a crossing. Ideally we
// want to find footpaths parallel(ish) to severances. If we had some kind of generalized edge
// bundling...
pub fn along_severances(map: &mut MapModel, n: usize) -> FeatureCollection {
    let mut requests = Vec::new();
    for r in &map.roads {
        if r.kind != RoadKind::Severance || !r.tags.is("name:en", "Waterloo Road") {
            continue;
        }
        for line in make_perpendicular_offsets(&r.linestring, 25.0, 15.0) {
            requests.push(line.into());
        }
    }
    calculate(map, requests)
}

fn calculate(map: &mut MapModel, requests: Vec<CompareRouteRequest>) -> FeatureCollection {
    let mut samples = Vec::new();
    let mut max_score = 0.0_f64;
    for req in requests {
        let mut f = Feature::from(geojson::Geometry::from(&LineString::new(vec![
            (req.x1, req.y1).into(),
            (req.x2, req.y2).into(),
        ])));
        if let Ok(fc) = crate::route::do_route(map, req) {
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
fn make_perpendicular_offsets(
    linestring: &LineString,
    walk_every_m: f64,
    project_away_m: f64,
) -> Vec<Line> {
    let mut output = Vec::new();
    // Using lines instead of coords so we can get the angle -- but is this hard to reason about?
    // angle_at_point instead?
    for orig_line in linestring.densify_haversine(walk_every_m).lines() {
        // TODO For the last line, use the last point too
        let pt: Point = orig_line.start.into();
        let angle = pt.haversine_bearing(orig_line.end.into());
        let projected_left = pt.haversine_destination(angle - 90.0, project_away_m);
        let projected_right = pt.haversine_destination(angle + 90.0, project_away_m);
        output.push(Line::new(projected_left, projected_right));
    }
    output
}
