use std::collections::HashMap;

use geo::{Coord, LineString};
use graph::RoadID;

// TODO For simplicty right now, hardcodes types. Make generic later.
// TODO Upstream in geo or utils

/// A linestring with a list of IDs in order
pub struct KeyedLineString {
    pub linestring: LineString,
    // True if forwards, false if backwards
    pub ids: Vec<(RoadID, bool)>,
}

impl KeyedLineString {
    fn first_pt(&self) -> HashedPoint {
        HashedPoint::new(*self.linestring.0.first().unwrap())
    }

    fn last_pt(&self) -> HashedPoint {
        HashedPoint::new(*self.linestring.0.last().unwrap())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HashedPoint(isize, isize);

impl HashedPoint {
    fn new(pt: Coord) -> Self {
        // cm precision
        Self((pt.x * 100.0) as isize, (pt.y * 100.0) as isize)
    }
}

/// Find all linestrings that meet at one end and join them
pub fn collapse_degree_2(input_lines: Vec<KeyedLineString>) -> Vec<KeyedLineString> {
    // Assign each input an ID that doesn't change
    let mut lines: HashMap<usize, KeyedLineString> = input_lines.into_iter().enumerate().collect();
    let mut id_counter = lines.len();

    // TODO Do this in one pass, either with something like union-find, just a search starting from
    // any degree 2, or by updating point_to_line indices
    loop {
        // How many lines connect to each point?
        let mut point_to_line: HashMap<HashedPoint, Vec<usize>> = HashMap::new();
        for (id, line) in &lines {
            point_to_line
                .entry(line.first_pt())
                .or_insert_with(Vec::new)
                .push(*id);
            point_to_line
                .entry(line.last_pt())
                .or_insert_with(Vec::new)
                .push(*id);
        }

        // Find any degree 2 case
        let Some(pair) = point_to_line.into_values().find(|list| list.len() == 2) else {
            break;
        };
        let (idx1, idx2) = (pair[0], pair[1]);

        let line1 = lines.remove(&idx1).unwrap();
        let line2 = lines.remove(&idx2).unwrap();
        let joined = join_lines(line1, line2);
        lines.insert(id_counter, joined);
        id_counter += 1;
    }

    lines.into_values().collect()
}

fn join_lines(mut line1: KeyedLineString, mut line2: KeyedLineString) -> KeyedLineString {
    let (pt1, pt2) = (line1.first_pt(), line1.last_pt());
    let (pt3, pt4) = (line2.first_pt(), line2.last_pt());

    if pt1 == pt3 {
        line1.linestring.0.reverse();
        line1.linestring.0.pop();
        line1.linestring.0.extend(line2.linestring.0);

        line1.ids.reverse();
        flip_direction(&mut line1.ids);
        line1.ids.extend(line2.ids);
    } else if pt1 == pt4 {
        line2.linestring.0.pop();
        line2.linestring.0.extend(line1.linestring.0);
        line1.linestring.0 = line2.linestring.0;

        line2.ids.extend(line1.ids);
        line1.ids = line2.ids;
    } else if pt2 == pt3 {
        line1.linestring.0.pop();
        line1.linestring.0.extend(line2.linestring.0);

        line1.ids.extend(line2.ids);
    } else if pt2 == pt4 {
        line2.linestring.0.reverse();
        line1.linestring.0.pop();
        line1.linestring.0.extend(line2.linestring.0);

        line2.ids.reverse();
        flip_direction(&mut line2.ids);
        line1.ids.extend(line2.ids);
    } else {
        unreachable!()
    }

    line1
}

fn flip_direction(ids: &mut Vec<(RoadID, bool)>) {
    for pair in ids {
        pair.1 = !pair.1;
    }
}
