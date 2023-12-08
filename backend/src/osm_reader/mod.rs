mod pbf;
mod xml;

use std::collections::HashMap;
use std::fmt;

use anyhow::Result;

pub use self::pbf::parse_pbf;
pub use self::xml::parse_xml;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct NodeID(pub i64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WayID(pub i64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RelationID(pub i64);

impl fmt::Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/node/{}", self.0)
    }
}
impl fmt::Display for WayID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/way/{}", self.0)
    }
}
impl fmt::Display for RelationID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "https://www.openstreetmap.org/relation/{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum OsmID {
    Node(NodeID),
    Way(WayID),
    Relation(RelationID),
}

impl fmt::Display for OsmID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsmID::Node(n) => write!(f, "{}", n),
            OsmID::Way(w) => write!(f, "{}", w),
            OsmID::Relation(r) => write!(f, "{}", r),
        }
    }
}

// TODO Into for both directions

pub enum Element {
    Node {
        id: NodeID,
        lon: f64,
        lat: f64,
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

// Per https://wiki.openstreetmap.org/wiki/OSM_XML#Certainties_and_Uncertainties, we assume
// elements come in order: nodes, ways, then relations.
pub fn parse(input_bytes: &[u8]) -> Result<Vec<Element>> {
    if is_xml(input_bytes) {
        info!("Got {} bytes, looks like XML", input_bytes.len());
        parse_xml(input_bytes)
    } else {
        info!(
            "Got {} bytes, assuming PBF (doesn't look like XML)",
            input_bytes.len()
        );
        parse_pbf(input_bytes)
    }
}

fn is_xml(input_bytes: &[u8]) -> bool {
    let check_header = "<?xml";
    if input_bytes.len() < check_header.len() {
        return false;
    }
    if let Ok(x) = std::str::from_utf8(&input_bytes[..check_header.len()]) {
        return x.to_lowercase() == check_header;
    }
    false
}
