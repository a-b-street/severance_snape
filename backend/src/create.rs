use std::collections::{HashMap, HashSet};
use std::time::Duration;

use anyhow::Result;
use geo::{Coord, LineString, Polygon};
use graph::{Direction, Graph, RoadID, Timer};
use osm_reader::{NodeID, OsmID, RelationID, WayID};
use utils::Tags;
use utils::osm2graph::OsmReader;

use crate::{Crossing, CrossingKind, MapModel, Profile, RoadKind, Settings, cost};

impl MapModel {
    pub fn create(input_bytes: &[u8], profile: Profile) -> Result<Self> {
        let mut extra_scraped = CrossingsAndBuildings::default();
        let graph = Graph::new(
            input_bytes,
            &mut extra_scraped,
            post_process_graph(profile),
            scrape_graph(profile),
            vec![
                ("walking".to_string(), walking_profile(profile)),
                ("cross_anywhere".to_string(), cross_anywhere(profile)),
            ],
            &mut Timer::new("build graph", None),
        )?;

        let road_kinds = graph
            .roads
            .iter()
            .map(|r| profile.classify(&r.osm_tags).unwrap())
            .collect();
        let gradients = std::iter::repeat(0.0).take(graph.roads.len()).collect();

        let crossings = extra_scraped
            .crossings
            .into_iter()
            .map(|(osm_id, pt, tags, roads)| Crossing {
                kind: CrossingKind::from_tags(&tags),
                osm_id,
                point: graph.mercator.pt_to_mercator(pt),
                roads,
                tags,
            })
            .collect();

        // TODO Careful with this; do we need two closest roads for the two profiles?
        let profile = graph.profile_names["cross_anywhere"];
        let mut buildings_per_road = HashMap::new();
        for mut b in extra_scraped.buildings {
            graph.mercator.to_mercator_in_place(&mut b);
            // TODO Use the centroid?
            let r = graph
                .snap_to_road(*b.exterior().coords().next().unwrap(), profile)
                .road;
            buildings_per_road.entry(r).or_insert_with(Vec::new).push(b);
        }

        Ok(Self {
            graph,
            road_kinds,
            crossings,
            gradients,

            buildings_per_road,

            walking_settings: Settings::uk(),
            cross_anywhere_settings: Settings {
                obey_crossings: false,
                ..Settings::uk()
            },
        })
    }
}

#[derive(Default)]
struct CrossingsAndBuildings {
    crossings: Vec<(NodeID, Coord, Tags, HashSet<RoadID>)>,
    buildings: Vec<Polygon>,
}

impl OsmReader for CrossingsAndBuildings {
    fn node(&mut self, id: NodeID, pt: Coord, tags: Tags) {
        if tags.is("highway", "crossing")
            || (tags.is("highway", "traffic_signals") && tags.is("crossing", "traffic_signals"))
        {
            self.crossings.push((id, pt, tags, HashSet::new()));
        } else if tags.has("crossing") {
            warn!("Ignoring possible crossing {id:?} with tags {tags:?}");
        }
    }

    fn way(
        &mut self,
        _: WayID,
        node_ids: &Vec<NodeID>,
        node_mapping: &HashMap<NodeID, Coord>,
        tags: &Tags,
    ) {
        if tags.has("building") {
            // TODO Handle relations, and refactor this
            // geo closes the polygon for us
            self.buildings.push(Polygon::new(
                LineString::new(node_ids.into_iter().map(|id| node_mapping[id]).collect()),
                Vec::new(),
            ));
        }
    }

    fn relation(&mut self, _: RelationID, _: &Vec<(String, OsmID)>, _: &Tags) {}
}

fn walking_profile(profile: Profile) -> Box<dyn Fn(&Tags, &LineString) -> (Direction, Duration)> {
    Box::new(move |tags, linestring| {
        let exclude = (Direction::None, Duration::ZERO);
        let kind = profile.classify(tags);
        if kind == None || kind == Some(RoadKind::Severance) {
            return exclude;
        }

        // TODO We haven't calculated it yet
        let gradient = 0.0;
        let (cost1, cost2) = cost(linestring, kind.unwrap(), gradient, &Settings::uk());
        (Direction::Both, cost1 + cost2)
    })
}

fn cross_anywhere(profile: Profile) -> Box<dyn Fn(&Tags, &LineString) -> (Direction, Duration)> {
    Box::new(move |tags, linestring| {
        let exclude = (Direction::None, Duration::ZERO);
        let kind = profile.classify(tags);
        if kind == None {
            return exclude;
        }

        // TODO We haven't calculated it yet
        let gradient = 0.0;
        let (cost1, cost2) = cost(linestring, kind.unwrap(), gradient, &Settings::uk());
        (Direction::Both, cost1 + cost2)
    })
}

fn scrape_graph(
    profile: Profile,
) -> Box<dyn Fn(&mut CrossingsAndBuildings, &utils::osm2graph::Graph) -> Result<()>> {
    Box::new(move |extra_scraped, graph| {
        // Only keep crossings on severances
        let mut severance_nodes: HashMap<NodeID, HashSet<RoadID>> = HashMap::new();
        for edge in graph.edges.values() {
            if profile.classify(&edge.osm_tags) == Some(RoadKind::Severance) {
                for node in &edge.node_ids {
                    // EdgeID becomes RoadID, because IDs have already been compacted
                    severance_nodes
                        .entry(*node)
                        .or_insert_with(HashSet::new)
                        .insert(RoadID(edge.id.0));
                }
            }
        }

        let mut keep_crossings = Vec::new();
        for mut crossing in extra_scraped.crossings.drain(..) {
            if let Some(roads) = severance_nodes.get(&crossing.0) {
                crossing.3.extend(roads.into_iter().cloned());
                keep_crossings.push(crossing);
            }
        }
        extra_scraped.crossings = keep_crossings;
        Ok(())
    })
}

fn post_process_graph(profile: Profile) -> Box<dyn Fn(&mut utils::osm2graph::Graph) -> Result<()>> {
    Box::new(move |graph| {
        // Look for intersections with only Severances and WithTraffic streets. Assume footways
        // directly connecting to severances (on both sides) are mistakenly tagged crossings.
        let mut disconnect: Vec<(utils::osm2graph::IntersectionID, utils::osm2graph::EdgeID)> =
            Vec::new();

        'INTERSECTION: for intersection in graph.intersections.values() {
            let mut severances = Vec::new();
            let mut with_traffics = Vec::new();
            for e in &intersection.edges {
                let kind = profile.classify(&graph.edges[e].osm_tags);
                match kind {
                    Some(RoadKind::Severance) => {
                        severances.push(*e);
                    }
                    Some(RoadKind::WithTraffic) => {
                        with_traffics.push(*e);
                    }
                    Some(RoadKind::Footway | RoadKind::Crossing(_)) => {
                        // TODO What if there are mixes involving WithTraffic too?
                        continue 'INTERSECTION;
                    }
                    None => {}
                }
            }
            // TODO Check that we actually have to cross the severance, and that it's not just
            // connected on one side
            if !severances.is_empty() && !with_traffics.is_empty() {
                for e in with_traffics {
                    disconnect.push((intersection.id, e));
                }
            }
        }

        // Don't allow connections between Severances and WithTraffics. This will apply for
        // routing, isochrones, network disconnections, etc. Achieve this by duplicating the
        // Intersection.
        for (original_node, edge_id) in disconnect {
            graph
                .intersections
                .get_mut(&original_node)
                .unwrap()
                .edges
                .retain(|e| *e != edge_id);

            let new_intersection_id = new_intersection_id(graph);
            assert!(!graph.intersections.contains_key(&new_intersection_id));
            let mut intersection_copy = graph.intersections[&original_node].clone();
            intersection_copy.edges = vec![edge_id];
            // TODO Keep same osm_node?
            intersection_copy.id = new_intersection_id;
            graph
                .intersections
                .insert(new_intersection_id, intersection_copy);

            let edge = graph.edges.get_mut(&edge_id).unwrap();
            if edge.src == original_node {
                edge.src = new_intersection_id;
            }
            if edge.dst == original_node {
                edge.dst = new_intersection_id;
            }
        }

        Ok(())
    })
}

fn new_intersection_id(graph: &utils::osm2graph::Graph) -> utils::osm2graph::IntersectionID {
    utils::osm2graph::IntersectionID(graph.intersections.keys().max().unwrap().0 + 1)
}
