mod race;

use aoc_2023::{show_solutions, Solution};
use itertools::Itertools;
use race::Race;

fn main() {
    let input = include_str!("../../../puzzle-input/day6.txt");

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
        let (times, distances) = self.0.split_once('\n').unwrap();
        let times = times
            .strip_prefix("Time:")
            .and_then(|str| {
                str.split_whitespace()
                    .map(|str| str.parse().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .expect("failed to parse times");

        let distances = distances
            .strip_prefix("Distance:")
            .and_then(|str| {
                str.split_whitespace()
                    .map(|str| str.parse().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .expect("failed to parse distances");

        let races = times
            .into_iter()
            .zip_eq(distances.into_iter())
            .map(|time_and_dist| Race::from(time_and_dist));

        races.map(|race| race.num_ways_to_win()).product()
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
        let (time, distance) = self.0.split_once('\n').unwrap();
        let time = time
            .strip_prefix("Time:")
            .and_then(|str| str.split_whitespace().join("").parse().ok())
            .expect("failed to parse times");

        let distance = distance
            .strip_prefix("Distance:")
            .and_then(|str| str.split_whitespace().join("").parse::<u64>().ok())
            .expect("failed to parse distances");

        let race = Race::from((time, distance));
        race.num_ways_to_win()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part1() {
        let input = "Time: 7 15 30\nDistance: 9 40 200";
        let expected = 288;
        let solution = PartOne::new(input).solve();

        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "Time: 7 15 30\nDistance: 9 40 200";
        let expected = 71503;
        let solution = PartTwo::new(input).solve();

        assert_eq!(solution, expected);
    }
}
