use std::time::Duration;

use chrono::NaiveTime;
use geo::{Coord, Densify, Euclidean, Rect};
use geojson::{Feature, GeoJson};
use graph::Graph;
use serde::Deserialize;
use utils::Grid;

use crate::{MapModel, Settings};

impl MapModel {
    pub fn calculate_isochrone(
        &mut self,
        start: Coord,
        style: Style,
        time_limit_mins: u64,
        settings: Settings,
    ) -> GeoJson {
        let profile = self.prepare_profile(settings.clone());
        let start = self.graph.snap_to_road(start, profile);

        let public_transit = false;
        let start_time = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        let limit = Duration::from_secs(time_limit_mins * 60);
        let cost_per_road = self.graph.get_costs(
            vec![start.intersection],
            profile,
            public_transit,
            start_time,
            start_time + limit,
        );

        let mut features = Vec::new();
        // Show reached amenities
        /*for (r, _) in &cost_per_road {
            for a in &amenities.per_road[r.0][profile.0] {
                features.push(amenities.amenities[a.0].to_gj(&graph.mercator));
            }
        }*/

        match style {
            Style::Roads => {
                for (r, cost) in cost_per_road {
                    let mut f = self
                        .graph
                        .mercator
                        .to_wgs84_gj(&self.graph.roads[r.0].linestring);
                    f.set_property("cost_seconds", cost.as_secs());
                    features.push(f);
                }
            }
            Style::Dasymetric => {
                let empty = Vec::new();
                for (r, cost) in cost_per_road {
                    for polygon in self.buildings_per_road.get(&r).unwrap_or(&empty) {
                        let mut f = self.graph.mercator.to_wgs84_gj(polygon);
                        f.set_property("cost_seconds", cost.as_secs());
                        features.push(f);
                    }
                }
            }
            Style::Grid | Style::Contours => {
                // Grid values are cost in seconds
                let mut grid: Grid<f64> = Grid::new(
                    (self.graph.mercator.width / RESOLUTION_M).ceil() as usize,
                    (self.graph.mercator.height / RESOLUTION_M).ceil() as usize,
                    0.0,
                );

                for (r, cost) in cost_per_road {
                    for pt in Euclidean
                        .densify(&self.graph.roads[r.0].linestring, RESOLUTION_M / 2.0)
                        .0
                    {
                        let grid_idx = grid.idx(
                            (pt.x / RESOLUTION_M) as usize,
                            (pt.y / RESOLUTION_M) as usize,
                        );
                        // If there are overlapping grid cells (bridges, tunnels, precision), just blindly
                        // clobber
                        grid.data[grid_idx] = cost.as_secs_f64();
                    }
                }

                if matches!(style, Style::Grid) {
                    features.extend(render_grid(&self.graph, grid));
                } else {
                    //features.extend(render_contours(graph, grid));
                }
            }
        }

        GeoJson::from(features)
    }
}

#[derive(Deserialize)]
pub enum Style {
    Roads,
    Grid,
    Contours,
    Dasymetric,
}

const RESOLUTION_M: f64 = 100.0;

/*fn render_contours(graph: &Graph, grid: Grid<f64>) -> Vec<Feature> {
    let smooth = false;
    let contour_builder = contour::ContourBuilder::new(grid.width, grid.height, smooth)
        .x_step(RESOLUTION_M)
        .y_step(RESOLUTION_M);
    let thresholds = vec![3. * 60., 6. * 60., 9. * 60., 12. * 60., 15. * 60.];

    let mut features = Vec::new();
    for band in contour_builder.isobands(&grid.data, &thresholds).unwrap() {
        let mut f = Feature::from(Geometry::from(&graph.mercator.to_wgs84(band.geometry())));
        f.set_property("min_seconds", band.min_v());
        f.set_property("max_seconds", band.max_v());
        features.push(f);
    }
    features
}*/

fn render_grid(graph: &Graph, grid: Grid<f64>) -> Vec<Feature> {
    let mut features = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            let value = grid.data[grid.idx(x, y)];
            if value == 0.0 {
                continue;
            }

            let rect = Rect::new(
                Coord {
                    x: (x as f64) * RESOLUTION_M,
                    y: (y as f64) * RESOLUTION_M,
                },
                Coord {
                    x: ((x + 1) as f64) * RESOLUTION_M,
                    y: ((y + 1) as f64) * RESOLUTION_M,
                },
            )
            .to_polygon();
            let mut f = graph.mercator.to_wgs84_gj(&rect);
            let step = 3.0 * 60.0;
            let min = step * (value / step).floor();
            f.set_property("cost_seconds", min);
            features.push(f);
        }
    }

    features
}
