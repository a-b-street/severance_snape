use std::collections::{BTreeSet, HashMap};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HashedPoint(isize, isize);

// Unrelated to the input type
#[derive(Clone, Copy, PartialEq, Eq)]
struct RoadIDx(usize);

impl HashedPoint {
    fn new(pt: Coord) -> Self {
        // cm precision
        Self((pt.x * 100.0) as isize, (pt.y * 100.0) as isize)
    }
}

/// Find all linestrings that meet at one end and join them
pub fn collapse_degree_2(mut lines: Vec<KeyedLineString>) -> Vec<KeyedLineString> {
    // TODO I think this is doable in one pass
    loop {
        let mut intersections: HashMap<HashedPoint, RoadIDx> = HashMap::new();

        let mut path = None;
        'FIND: for (idx1, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.linestring.0.first().unwrap());
            let i2 = HashedPoint::new(*line.linestring.0.last().unwrap());
            if i1 == i2 {
                continue;
            }

            let idx1 = RoadIDx(idx1);
            for i in [i1, i2] {
                match intersections.get(&i) {
                    Some(idx2) => {
                        // Don't create a loop though!
                        // TODO Doesn't seem to always work
                        if number_shared_endpoints(line, &lines[idx2.0]) == 1 {
                            path = Some(vec![idx1, *idx2]);
                            break 'FIND;
                        }
                    }
                    None => {
                        intersections.insert(i, idx1);
                    }
                }
            }
        }

        if let Some(path) = path {
            lines = join_path(lines, path);
        } else {
            break;
        }
    }
    lines
}

// Combines everything in the path, returning a smaller list of lines
fn join_path(lines: Vec<KeyedLineString>, path: Vec<RoadIDx>) -> Vec<KeyedLineString> {
    // Build up the joined line
    let mut points = Vec::new();
    let mut ids = Vec::new();

    for idx in &path {
        let mut next_ids = lines[idx.0].ids.clone();
        let mut next_points = lines[idx.0].linestring.clone().into_inner();

        if points.is_empty() {
            points = next_points;
            ids = next_ids;
            continue;
        }
        let pt1 = HashedPoint::new(*points.first().unwrap());
        let pt2 = HashedPoint::new(*points.last().unwrap());
        let pt3 = HashedPoint::new(*next_points.first().unwrap());
        let pt4 = HashedPoint::new(*next_points.last().unwrap());

        if pt1 == pt3 {
            points.reverse();
            points.pop();
            points.extend(next_points);

            ids.reverse();
            flip_direction(&mut ids);
            ids.extend(next_ids);
        } else if pt1 == pt4 {
            next_points.pop();
            next_points.extend(points);
            points = next_points;

            next_ids.extend(ids);
            ids = next_ids;
        } else if pt2 == pt3 {
            points.pop();
            points.extend(next_points);

            ids.extend(next_ids);
        } else if pt2 == pt4 {
            next_points.reverse();
            points.pop();
            points.extend(next_points);

            next_ids.reverse();
            flip_direction(&mut next_ids);
            ids.extend(next_ids);
        } else {
            unreachable!()
        }
    }

    let mut result = vec![KeyedLineString {
        linestring: LineString::new(points),
        ids,
    }];

    // Leftovers
    for (i, line) in lines.into_iter().enumerate() {
        if !path.contains(&RoadIDx(i)) {
            result.push(line);
        }
    }
    result
}

fn number_shared_endpoints(line1: &KeyedLineString, line2: &KeyedLineString) -> usize {
    let mut set = BTreeSet::new();
    set.insert(HashedPoint::new(*line1.linestring.0.first().unwrap()));
    set.insert(HashedPoint::new(*line1.linestring.0.last().unwrap()));
    set.insert(HashedPoint::new(*line2.linestring.0.first().unwrap()));
    set.insert(HashedPoint::new(*line2.linestring.0.last().unwrap()));
    4 - set.len()
}

fn flip_direction(ids: &mut Vec<(RoadID, bool)>) {
    for pair in ids {
        pair.1 = !pair.1;
    }
}
