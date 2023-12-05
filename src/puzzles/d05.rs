use self::map::Map;
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

    pub fn get_min_location_for_range(&self, range: Range<u64>) -> u64 {
        // Assuming the maps appear in order, with the map to location last
        let mut ranges = vec![range];
        for map in &self.maps {
            let mut transformed_ranges = Vec::new();
            for range in &ranges {
                transformed_ranges.append(&mut map.transform_range(range.clone()));
            }
            // TODO: Merge overlapping ranges for better performance (?)

            ranges = transformed_ranges;
        }

        ranges.iter().map(|r| r.start).min().unwrap()
    }
}

mod map {
    use std::{error::Error, ops::Range, str::FromStr};

    pub struct Map {
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
            ranges.sort_by_key(|r: &MapRange| r.from.start);

            Ok(Map { ranges })
        }
    }

    impl Map {
        pub fn transform(&self, x: u64) -> u64 {
            // Assume the ranges do not overlap, so return on the first hit
            for rn in &self.ranges {
                if rn.from.contains(&x) {
                    return rn.to_start + x - rn.from.start;
                }
            }
            x
        }

        pub fn transform_range(&self, mut range: Range<u64>) -> Vec<Range<u64>> {
            let mut transformed = Vec::new();

            // note that self.ranges is sorted by from.start
            for mr in &self.ranges {
                if range.end <= mr.from.start {
                    transformed.push(range);
                    return transformed;
                } else if range.start < mr.from.start && range.end <= mr.from.end {
                    transformed.push(range.start..mr.from.start);
                    transformed.push(mr.to_start..(mr.to_start + range.end - mr.from.start));
                    return transformed;
                } else if range.start < mr.from.start {
                    // and it must be that range.end > mr.from.end
                    transformed.push(range.start..mr.from.start);
                    transformed.push(mr.to_start..(mr.to_start + mr.len()));
                    range = mr.from.end..range.end;
                } else if range.start < mr.from.end && range.end <= mr.from.end {
                    transformed.push(
                        (mr.to_start + range.start - mr.from.start)
                            ..(mr.to_start + range.end - mr.from.start),
                    );
                    return transformed;
                } else if range.start < mr.from.end {
                    // and it must be that range.end > mr.from.end
                    transformed.push(
                        (mr.to_start + range.start - mr.from.start)..(mr.to_start + mr.len()),
                    );
                    range = mr.from.end..range.end;
                }
            }

            transformed.push(range);
            transformed
        }
    }

    struct MapRange {
        from: Range<u64>,
        to_start: u64,
    }

    impl MapRange {
        fn len(&self) -> u64 {
            self.from.end - self.from.start
        }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_transform_range() {
        let map: Map = concat!(
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 55 2\n",
            "39 0 15\n",
        )
        .parse()
        .unwrap();

        assert_eq!(map.transform_range(0..15), vec![39..54]);
        assert_eq!(map.transform_range(0..16), vec![39..54, 0..1]);
        assert_eq!(map.transform_range(0..52), vec![39..54, 0..37]);
        assert_eq!(
            map.transform_range(0..60),
            vec![39..54, 0..37, 52..55, 37..39, 57..60]
        );
        assert_eq!(map.transform_range(40..56), vec![25..37, 52..55, 37..38]);
    }
}
