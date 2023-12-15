use std::{ops::Range, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
pub struct Almanac {
    pub maps: Vec<Map>,
}

impl FromIterator<Map> for Almanac {
    fn from_iter<T: IntoIterator<Item = Map>>(iter: T) -> Self {
        let maps = iter.into_iter().collect();
        Self { maps }
    }
}

#[derive(Debug)]
pub struct Map {
    pub ranges: Vec<MapRange>,
}

impl Map {
    pub fn map_to_next(&self, seed: u64) -> u64 {
        match self.ranges.iter().find(|r| r.contains(&seed)) {
            Some(range) => range.translate(seed),
            None => seed,
        }
    }

    pub fn with_implicit_empty_ranges(self) -> Self {
        let mut empty_ranges = Vec::new();

        for (a, b) in self.ranges.iter().tuple_windows() {
            if b.src.start - a.src.end > 0 {
                empty_ranges.push(MapRange {
                    src: a.src.end..b.src.start,
                    dest: a.src.end..b.src.start,
                })
            }
        }

        Self {
            ranges: self
                .ranges
                .into_iter()
                .chain(empty_ranges.into_iter())
                .sorted_by(|a, b| Ord::cmp(&a.src.start, &b.src.start))
                .collect(),
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .filter(|line| !line.ends_with("map:"))
            .filter_map(|line| {
                let (dest_start, src_start, length) = line
                    .trim()
                    .split_whitespace()
                    .filter_map(|str| str.parse().ok())
                    .collect_tuple()?;

                Some(MapRange {
                    src: src_start..src_start + length,
                    dest: dest_start..dest_start + length,
                })
            })
            .sorted_by(|a, b| Ord::cmp(&a.src.start, &b.src.start))
            .collect::<Vec<_>>();

        Ok(Map { ranges })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MapRange {
    pub src: Range<u64>,
    pub dest: Range<u64>,
}

impl MapRange {
    pub fn contains(&self, seed: &u64) -> bool {
        self.src.contains(seed)
    }

    pub fn translate(&self, seed: u64) -> u64 {
        if self.contains(&seed) {
            let diff = seed - self.src.start;
            self.dest.start + diff
        } else {
            seed
        }
    }

    pub fn translate_range(&self, seed_range: &Range<u64>) -> Option<Range<u64>> {
        if self.contains(&seed_range.start) {
            let start = self.translate(seed_range.start);
            let end = self
                .contains(&seed_range.end)
                .then(|| self.translate(seed_range.end))
                .unwrap_or_else(|| self.dest.end);

            Some(start..end)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map() {
        let input = "50 98 2\n52 50 48";

        let map = input.parse::<Map>().unwrap();

        assert_eq!(map.map_to_next(98), 50);
        assert_eq!(map.map_to_next(99), 51);
        assert_eq!(map.map_to_next(10), 10);
        assert_eq!(map.map_to_next(50), 52);
        assert_eq!(map.map_to_next(53), 55);
    }

    #[test]
    fn test_empty_ranges() {
        let input = "50 98 2\n52 50 38";
        let expected = Map {
            ranges: vec![
                MapRange {
                    src: 50..88,
                    dest: 52..90,
                },
                MapRange {
                    src: 88..98,
                    dest: 88..98,
                },
                MapRange {
                    src: 98..100,
                    dest: 50..52,
                },
            ],
        };
        let map = input.parse::<Map>().unwrap().with_implicit_empty_ranges();

        assert_eq!(map.ranges, expected.ranges);

        let input = "50 98 2\n52 50 48";
        let expected = Map {
            ranges: vec![
                MapRange {
                    src: 50..98,
                    dest: 52..100,
                },
                MapRange {
                    src: 98..100,
                    dest: 50..52,
                },
            ],
        };
        let map = input.parse::<Map>().unwrap().with_implicit_empty_ranges();

        assert_eq!(map.ranges, expected.ranges);
    }
}
