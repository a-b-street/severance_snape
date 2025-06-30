use serde::Deserialize;
use utils::Tags;

use crate::RoadKind;

#[derive(Clone, Copy, PartialEq, Deserialize)]
pub enum Profile {
    SeparateWays,
    SidewalksOnHighways,
    USA,
    USAShoulders,
}

impl Profile {
    /// This function classifies an OSM way as a RoadKind. If it returns `None`, then the way is
    /// totally excluded from the walking graph.
    pub fn classify(self, tags: &Tags) -> Option<RoadKind> {
        if !tags.has("highway")
            || tags.is_any("highway", vec!["construction", "proposed"])
            || tags.is("area", "yes")
        {
            return None;
        }

        // Some kind of explicit footway
        if tags.is_any(
            "highway",
            vec!["footway", "steps", "path", "track", "corridor"],
        ) {
            if tags.is("footway", "crossing") {
                return Some(RoadKind::Crossing);
            }
            return Some(RoadKind::Footway);
        }
        if tags.is("highway", "cycleway") && tags.is("foot", "yes") {
            return Some(RoadKind::Footway);
        }

        if tags.is("highway", "crossing") || tags.has("crossing") {
            return Some(RoadKind::Crossing);
        }

        if self == Profile::USA {
            return usa(tags);
        }
        if self == Profile::USAShoulders {
            return usa_shoulders(tags);
        }

        // Big roads are always severances.
        // TODO Big roads without separate sidewalks aren't walkable at all right now.
        // https://github.com/dabreegster/severance_snape/issues/5
        if tags.is_any(
            "highway",
            vec![
                "motorway",
                "motorway_link",
                "trunk",
                "trunk_link",
                "primary",
                "primary_link",
            ],
        ) {
            return Some(RoadKind::Severance);
        }

        // Totally exclude roads that claim to have a separately mapped sidewalk; they're just noise.
        // I'm assuming there isn't a silly mix like "sidewalk:left = separate, sidewalk:right = yes".
        if tags.is("sidewalk", "separate")
            || tags.is("sidewalk:left", "separate")
            || tags.is("sidewalk:right", "separate")
            || tags.is("sidewalk:both", "separate")
        {
            return None;
        }

        if tags.is("highway", "pedestrian")
            || tags.is_any("sidewalk", vec!["both", "right", "left"])
        {
            return Some(RoadKind::WithTraffic);
        }

        // No sidewalk tagging. We can make a guess about which ones are still routable for walking. In
        // places with thoroughly tagged sidewalks, disable this. Keeping this on is usually messy,
        // because there'll be a mix of separately mapped RoadKind::Footways and then one of these
        // RoadKind::WithTraffic in the middle.
        if tags.is_any(
            "highway",
            vec![
                "secondary",
                "secondary_link",
                "tertiary",
                "tertiary_link",
                "residential",
                "unclassified",
                "service",
                "living_street",
            ],
        ) && !tags.is("foot", "no")
        {
            return match self {
                Profile::SeparateWays => None,
                Profile::SidewalksOnHighways => Some(RoadKind::WithTraffic),
                Profile::USA | Profile::USAShoulders => unreachable!(),
            };
        }

        // TODO Private?

        // TODO Maybe just use tagged / assumed speed limit instead of highway classification?

        // TODO wait, why's this the fallback case?
        Some(RoadKind::Severance)
    }
}

// Footway cases already handled
fn usa(tags: &Tags) -> Option<RoadKind> {
    if tags.is_any(
        "highway",
        vec![
            "motorway",
            "motorway_link",
            "trunk",
            "trunk_link",
            "primary",
            "primary_link",
            "secondary",
            "secondary_link",
            "tertiary",
            "tertiary_link",
        ],
    ) {
        return Some(RoadKind::Severance);
    }

    // Totally exclude roads that claim to have a separately mapped sidewalk; they're just noise.
    // I'm assuming there isn't a silly mix like "sidewalk:left = separate, sidewalk:right = yes".
    if tags.is("sidewalk", "separate")
        || tags.is("sidewalk:left", "separate")
        || tags.is("sidewalk:right", "separate")
        || tags.is("sidewalk:both", "separate")
    {
        return None;
    }

    // TODO
    Some(RoadKind::WithTraffic)
}

fn usa_shoulders(tags: &Tags) -> Option<RoadKind> {
    let kind = usa(tags)?;
    if kind == RoadKind::Severance
        && (tags.is("cycleway", "shoulder") || tags.is("cyclestreet", "yes"))
    {
        return Some(RoadKind::WithTraffic);
    }
    Some(kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usa() {
        // https://www.openstreetmap.org/way/1120402115
        // TODO Maybe this should be None? It really does have crossings, so not sure
        // It's similar to https://www.openstreetmap.org/way/571968303, except by speed and lane
        // count
        assert_eq!(
            Profile::USA.classify(&tags(vec!["highway=tertiary", "sidewalk=no",])),
            Some(RoadKind::Severance)
        );

        // https://www.openstreetmap.org/way/41945235
        assert_eq!(
            Profile::USA.classify(&tags(vec!["highway=secondary", "maxspeed=40 mph",])),
            Some(RoadKind::Severance)
        );
    }

    fn tags(kv: Vec<&str>) -> Tags {
        let mut tags = Tags::empty();
        for pair in kv {
            let parts = pair.split('=').collect::<Vec<_>>();
            tags.insert(parts[0], parts[1]);
        }
        tags
    }
}
