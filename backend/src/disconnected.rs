use std::collections::BTreeSet;

use geojson::FeatureCollection;
use petgraph::graphmap::UnGraphMap;

use crate::{IntersectionID, MapModel, RoadID, RoadKind};

pub fn find_connected_components(map: &MapModel) -> FeatureCollection {
    let mut graph: UnGraphMap<IntersectionID, RoadID> = UnGraphMap::new();
    for r in &map.roads {
        if r.kind != RoadKind::Severance {
            graph.add_edge(r.src_i, r.dst_i, r.id);
        }
    }

    let mut features = Vec::new();
    let mut component_sizes = Vec::new();
    for nodes in petgraph::algo::tarjan_scc(&graph) {
        let component = component_sizes.len();
        let roads = nodes_to_edges(map, nodes);
        component_sizes.push(roads.len());

        for r in roads {
            let mut f = map.roads[r.0].to_gj(&map.mercator);
            f.set_property("component", component);
            features.push(f);
        }
    }
    component_sizes.sort();
    component_sizes.reverse();

    FeatureCollection {
        features,
        bbox: None,
        foreign_members: Some(
            serde_json::json!({
                "components": component_sizes,
            })
            .as_object()
            .unwrap()
            .clone(),
        ),
    }
}

// Note this only works for connected components of nodes!
fn nodes_to_edges(map: &MapModel, nodes: Vec<IntersectionID>) -> BTreeSet<RoadID> {
    let mut edges = BTreeSet::new();
    for i in nodes {
        edges.extend(map.intersections[i.0].roads.clone());
    }
    edges.retain(|r| map.roads[r.0].kind != RoadKind::Severance);
    edges
}
