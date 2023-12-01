use std::collections::HashMap;

use anyhow::Result;
use geo::Coord;

use crate::osm::*;

// Per https://wiki.openstreetmap.org/wiki/OSM_XML#Certainties_and_Uncertainties, we assume
// elements come in order: nodes, ways, then relations.
pub fn parse_osm(input_bytes: &[u8]) -> Result<Vec<Element>> {
    info!("Got {} bytes", input_bytes.len());
    // TODO Detect file type

    parse_xml(input_bytes)
}

pub enum Element {
    Node {
        id: NodeID,
        pt: Coord,
        tags: HashMap<String, String>,
    },
    Way {
        id: WayID,
        node_ids: Vec<NodeID>,
        tags: HashMap<String, String>,
    },
    Relation {
        id: RelationID,
        tags: HashMap<String, String>,
        // Role, member ID
        members: Vec<(String, OsmID)>,
    },
}

// TODO Iterator instead
fn parse_xml(input_bytes: &[u8]) -> Result<Vec<Element>> {
    let input_string = String::from_utf8(input_bytes.to_vec())?;
    // TODO anyhow compatibility
    let tree = roxmltree::Document::parse(&input_string).unwrap();
    let mut elements = Vec::new();
    for obj in tree.descendants() {
        if !obj.is_element() {
            continue;
        }
        match obj.tag_name().name() {
            "bounds" => {
                // TODO Use this or not?
            }
            "node" => {
                let id = NodeID(obj.attribute("id").unwrap().parse::<i64>()?);
                let pt = Coord {
                    x: obj.attribute("lon").unwrap().parse::<f64>()?,
                    y: obj.attribute("lat").unwrap().parse::<f64>()?,
                };
                let tags = read_tags(obj);
                elements.push(Element::Node { id, pt, tags });
            }
            "way" => {
                let id = WayID(obj.attribute("id").unwrap().parse::<i64>()?);
                let tags = read_tags(obj);

                let mut node_ids = Vec::new();
                for child in obj.children() {
                    if child.tag_name().name() == "nd" {
                        let n = NodeID(child.attribute("ref").unwrap().parse::<i64>()?);
                        // TODO Check for missing nodes
                        node_ids.push(n);
                    }
                }
                elements.push(Element::Way { id, node_ids, tags });
            }
            "relation" => {
                let id = RelationID(obj.attribute("id").unwrap().parse::<i64>()?);
                let tags = read_tags(obj);
                let mut members = Vec::new();
                for child in obj.children() {
                    if child.tag_name().name() == "member" {
                        let member = match child.attribute("type").unwrap() {
                            "node" => {
                                let n =
                                    NodeID(child.attribute("ref").unwrap().parse::<i64>().unwrap());
                                OsmID::Node(n)
                            }
                            "way" => {
                                let w =
                                    WayID(child.attribute("ref").unwrap().parse::<i64>().unwrap());
                                OsmID::Way(w)
                            }
                            "relation" => {
                                let r = RelationID(
                                    child.attribute("ref").unwrap().parse::<i64>().unwrap(),
                                );
                                OsmID::Relation(r)
                            }
                            _ => continue,
                        };
                        members.push((child.attribute("role").unwrap().to_string(), member));
                    }
                }
                elements.push(Element::Relation { id, members, tags });
            }
            _ => {}
        }
    }
    Ok(elements)
}

fn read_tags(obj: roxmltree::Node) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    for child in obj.children() {
        if child.tag_name().name() == "tag" {
            let key = child.attribute("k").unwrap();
            tags.insert(key.to_string(), child.attribute("v").unwrap().to_string());
        }
    }
    tags
}
