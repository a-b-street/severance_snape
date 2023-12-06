use geo::{Rect, GeometryCollection, BoundingRect, Geometry, LineString};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use geojson::{Feature, FeatureCollection};

use crate::{CompareRouteRequest, MapModel};

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
    let dist_away = 0.1;

    let mut rng = XorShiftRng::seed_from_u64(42);
    let mut samples = Vec::new();
    for _ in 0..n {
        let x1 = rng.gen_range(bbox.min().x..=bbox.max().x);
        let y1 = rng.gen_range(bbox.min().y..=bbox.max().y);
        let x2 = x1 + rng.gen_range(-dist_away..=dist_away);
        let y2 = y1 + rng.gen_range(-dist_away..=dist_away);
        let req = CompareRouteRequest { x1, y1, x2, y2 };
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
            let mut f = Feature::from(geojson::Geometry::from(&LineString::new(vec![(x1, y1).into(), (x2, y2).into()])));
            f.set_property("score", score);
            samples.push(f);
        }
    }
    FeatureCollection {
        features: samples,
        bbox: None,
        foreign_members: None,
    }
}
