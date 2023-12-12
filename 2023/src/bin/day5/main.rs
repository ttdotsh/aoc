mod map;

use map::Mapper;

use aoc_2023::Solution;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../../puzzle-input/day5.txt");

    let part1 = PartOne::new(input);
    println!("{}", part1.solve());
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
            .filter_map(|(key, mut group)| match key {
                true => Some(group.join("\n")),
                false => None,
            })
            .filter_map(|str| str.parse().ok())
            .collect::<Mapper>();

        seeds
            .into_iter()
            .map(|seed| mapper.find_destination(seed))
            .min()
            .unwrap()
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
}
