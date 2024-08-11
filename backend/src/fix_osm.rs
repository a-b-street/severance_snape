use geo::Intersects;
use geojson::GeoJson;
use rstar::{RTree, RTreeObject};

use crate::MapModel;

pub fn find_separate_sidewalks(map: &MapModel) -> GeoJson {
    let footways = RTree::bulk_load(
        map.roads
            .iter()
            .filter(|r| r.tags.is("highway", "footway"))
            .map(|r| r.linestring.clone())
            .collect(),
    );

    let mut features = Vec::new();
    'ROAD: for r in &map.roads {
        if r.tags.is("highway", "footway")
            || r.tags.is("sidewalk", "separate")
            || r.tags.is("sidewalk:left", "separate")
            || r.tags.is("sidewalk:right", "separate")
            || r.tags.is("sidewalk:both", "separate")
        {
            continue;
        }

        // Along this road, if we project away, do we hit a footway?
        for line in crate::heatmap::make_perpendicular_offsets(&r.linestring, 25.0, 15.0) {
            for footway in footways.locate_in_envelope_intersecting(&line.envelope()) {
                if footway.intersects(&line) {
                    features.push(r.to_gj(&map.mercator));
                    continue 'ROAD;
                }
            }
        }
    }
    GeoJson::from(features)
}
