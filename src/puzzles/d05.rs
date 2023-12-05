use std::{error::Error, ops::Range, str::FromStr};

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");

        let seeds = parts
            .next()
            .ok_or("Missing first line")?
            .split_whitespace()
            .skip(1) // skip "seeds: "
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()?;

        let mut maps = Vec::new();
        for p in parts {
            maps.push(p.parse()?);
        }

        Ok(Almanac { seeds, maps })
    }
}

impl Almanac {
    pub fn seeds(&self) -> &[u64] {
        &self.seeds
    }

    pub fn get_location_num(&self, seed_num: u64) -> u64 {
        // Assuming the maps appear in order, with the map to location last
        self.maps.iter().fold(seed_num, |x, map| map.transform(x))
    }
}

struct Map {
    ranges: Vec<MapRange>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::new();

        // Skip the first line; assume all maps appear in order
        for line in s.lines().skip(1) {
            ranges.push(line.parse()?);
        }

        Ok(Map { ranges })
    }
}

impl Map {
    fn transform(&self, x: u64) -> u64 {
        // Assume the ranges do not overlap, so return on the first hit
        for rn in &self.ranges {
            if rn.from.contains(&x) {
                return rn.to_start + x - rn.from.start;
            }
        }
        x
    }
}

struct MapRange {
    from: Range<u64>,
    to_start: u64,
}

impl FromStr for MapRange {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_nums = s.split_whitespace();
        let to_start = str_nums.next().ok_or("Not enough numbers")?.parse()?;
        let from_start = str_nums.next().ok_or("Not enough numbers")?.parse()?;
        let len: u64 = str_nums.next().ok_or("Not enough numbers")?.parse()?;

        Ok(MapRange {
            from: from_start..(from_start + len),
            to_start,
        })
    }
}
