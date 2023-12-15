mod map;

use std::ops::Range;

use map::{Almanac, Map};

use aoc_2023::{show_solutions, Solution};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../../puzzle-input/day5.txt");

    let part1 = PartOne::new(input);
    let part2 = PartTwo::new(input);

    show_solutions(part1, part2);
}

struct PartOne<'a>(&'a str);

impl<'a> PartOne<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl Solution for PartOne<'_> {
    type Output = u64;

    fn solve(&self) -> Self::Output {
        let mut lines = self.0.trim().lines();

        let seeds = lines
            .next()
            .unwrap()
            .replace("seeds:", "")
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let mapper = lines
            .map(|line| line.trim())
            .group_by(|line| !line.is_empty())
            .into_iter()
            .filter_map(|(key, mut group)| key.then_some(group.join("\n")))
            .filter_map(|str| str.parse().ok())
            .collect::<Almanac>();

        seeds
            .into_iter()
            .map(|seed| mapper.maps.iter().fold(seed, |s, map| map.map_to_next(s)))
            .min()
            .unwrap()
    }
}

struct PartTwo<'a>(&'a str);

impl<'a> PartTwo<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl Solution for PartTwo<'_> {
    type Output = u64;

    fn solve(&self) -> Self::Output {
        let mut lines = self.0.trim().lines();

        let seed_ranges = lines
            .next()
            .unwrap()
            .replace("seeds:", "")
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .tuples()
            .map(|(start, len)| start..start + len)
            .collect::<Vec<Range<u64>>>();

        let mapper = lines
            .map(|line| line.trim())
            .group_by(|line| !line.is_empty())
            .into_iter()
            .filter_map(|(key, mut group)| key.then_some(group.join("\n")))
            .filter_map(|str| str.parse().ok())
            .map(|map: Map| map.with_implicit_empty_ranges())
            .collect::<Almanac>();

        seed_ranges
            .into_iter()
            .fold(std::u64::MAX, |mut lowest, mut seed_range| {
                while seed_range.end - seed_range.start > 0 {
                    let subrange =
                        mapper
                            .maps
                            .iter()
                            .fold(seed_range.start..seed_range.end, |seeds, map| {
                                map.ranges
                                    .iter()
                                    .find(|r| r.contains(&seeds.start))
                                    .and_then(|range| range.translate_range(&seeds))
                                    .unwrap_or_else(|| seeds)
                            });

                    let size = subrange.end - subrange.start;

                    seed_range.start += size;

                    if subrange.start < lowest {
                        lowest = subrange.start;
                    }
                }
                lowest
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part1() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";

        let expected = 35;

        let solution = PartOne::new(input).solve();

        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";

        let expected = 46;

        let solution = PartTwo::new(input).solve();

        assert_eq!(solution, expected);
    }
}
