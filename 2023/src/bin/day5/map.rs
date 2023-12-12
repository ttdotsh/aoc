use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct Mapper {
    maps: Vec<Map>,
}

impl Mapper {
    pub fn find_destination(&self, seed: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed, |s, map| map.find_next_destination(s))
    }
}

impl FromIterator<Map> for Mapper {
    fn from_iter<T: IntoIterator<Item = Map>>(iter: T) -> Self {
        let maps = iter.into_iter().collect();
        Self { maps }
    }
}

#[derive(Debug)]
pub struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn find_next_destination(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            match range.check_for_destination(source) {
                Some(dest) => return dest,
                None => (),
            }
        }
        source
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .filter(|line| !line.ends_with("map:"))
            .filter_map(|line| {
                let (dest_start, source_start, length) = line
                    .trim()
                    .split_whitespace()
                    .filter_map(|str| str.parse().ok())
                    .collect_tuple()?;

                Some(MapRange {
                    dest_start,
                    source_start,
                    length,
                })
            })
            .collect();

        Ok(Map { ranges })
    }
}

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl MapRange {
    fn check_for_destination(&self, source: u64) -> Option<u64> {
        let source_range = self.source_start..self.source_start + self.length;

        if source_range.contains(&source) {
            let diff = source - source_range.start;
            Some(self.dest_start + diff)
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

        assert_eq!(map.find_next_destination(98), 50);
        assert_eq!(map.find_next_destination(99), 51);
        assert_eq!(map.find_next_destination(10), 10);
        assert_eq!(map.find_next_destination(50), 52);
        assert_eq!(map.find_next_destination(53), 55);
    }
}
