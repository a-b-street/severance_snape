use std::collections::{HashMap, HashSet};
use std::time::Duration;

use anyhow::Result;
use geo::{Coord, Euclidean, Length, LineString};
use graph::{Direction, Graph, Timer};
use osm_reader::{NodeID, OsmID, RelationID, WayID};
use utils::osm2graph::OsmReader;
use utils::Tags;

use crate::{Crossing, MapModel, Profile, RoadKind};

impl MapModel {
    pub fn create(input_bytes: &[u8], profile: Profile) -> Result<Self> {
        let walking_profile = Box::new(move |tags: &Tags, linestring: &LineString| {
            let exclude = (Direction::None, Duration::ZERO);
            let kind = profile.classify(tags);
            if kind == None || kind == Some(RoadKind::Severance) {
                return exclude;
            }

            // 3mph
            let speed = 1.34112;
            let cost = Duration::from_secs_f64(Euclidean.length(linestring) / speed);
            (Direction::Both, cost)
        });
        // TODO Hack to include severances
        let dummy_profile = Box::new(move |tags: &Tags, _: &LineString| {
            if profile.classify(tags) == Some(RoadKind::Severance) {
                (Direction::Both, Duration::from_secs_f64(1.0))
            } else {
                (Direction::None, Duration::ZERO)
            }
        });

        let mut crossings = Crossings::default();
        let scrape_graph = Box::new(
            move |crossings: &mut Crossings, graph: &utils::osm2graph::Graph| {
                // Only keep crossings on severances
                let mut severance_nodes = HashSet::new();
                for edge in graph.edges.values() {
                    if profile.classify(&edge.osm_tags) == Some(RoadKind::Severance) {
                        severance_nodes.extend(edge.node_ids.clone());
                    }
                }

                crossings
                    .crossings
                    .retain(|(node, _)| severance_nodes.contains(node));
                Ok(())
            },
        );

        let graph = Graph::new(
            input_bytes,
            &mut crossings,
            Box::new(|_| Ok(())),
            scrape_graph,
            vec![
                ("walking".to_string(), walking_profile),
                ("dummy".to_string(), dummy_profile),
            ],
            &mut Timer::new("build graph", None),
        )?;

        let road_kinds = graph
            .roads
            .iter()
            .map(|r| profile.classify(&r.osm_tags).unwrap())
            .collect();

        let crossings = crossings
            .crossings
            .into_iter()
            .map(|(_, pt)| Crossing {
                point: graph.mercator.pt_to_mercator(pt),
            })
            .collect();

        Ok(Self {
            graph,
            road_kinds,
            crossings,
        })
    }
}

#[derive(Default)]
struct Crossings {
    crossings: Vec<(NodeID, Coord)>,
}

impl OsmReader for Crossings {
    fn node(&mut self, id: NodeID, pt: Coord, tags: Tags) {
        if tags.is("highway", "crossing") {
            self.crossings.push((id, pt));
        }
    }

    fn way(&mut self, _: WayID, _: &Vec<NodeID>, _: &HashMap<NodeID, Coord>, _: &Tags) {}

    fn relation(&mut self, _: RelationID, _: &Vec<(String, OsmID)>, _: &Tags) {}
}
