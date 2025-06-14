use geo::Intersects;
use geojson::GeoJson;
use rstar::{RTree, RTreeObject};

use crate::MapModel;

pub fn find_separate_sidewalks(map: &MapModel, duplicates_only: bool) -> GeoJson {
    let footways = RTree::bulk_load(
        map.graph
            .roads
            .iter()
            .filter(|r| r.osm_tags.is("highway", "footway"))
            .map(|r| r.linestring.clone())
            .collect(),
    );

    let mut features = Vec::new();
    'ROAD: for r in &map.graph.roads {
        if r.osm_tags.is("highway", "footway")
            || r.osm_tags.is("sidewalk", "separate")
            || r.osm_tags.is("sidewalk:left", "separate")
            || r.osm_tags.is("sidewalk:right", "separate")
            || r.osm_tags.is("sidewalk:both", "separate")
        {
            continue;
        }

        if duplicates_only && !r.osm_tags.is_any("sidewalk", vec!["left", "right", "both"]) {
            continue;
        }

        // Along this road, if we project away, do we hit a footway?
        for line in crate::scores::make_perpendicular_offsets(&r.linestring, 25.0, 15.0) {
            for footway in footways.locate_in_envelope_intersecting(&line.envelope()) {
                if footway.intersects(&line) {
                    features.push(r.to_gj(&map.graph));
                    continue 'ROAD;
                }
            }
        }
    }
    GeoJson::from(features)
}
