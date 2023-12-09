mod digit;

use aoc_2023::{show_solutions, Solution};
use digit::DigitParser;

fn main() {
    let input = include_str!("../../../puzzle-input/day1.txt");
    let part1 = PartOne::new(input);
    let part2 = PartTwo::new(input);

    show_solutions(part1, part2);
}

/*
* Part One
*/
struct PartOne<'a>(&'a str);

impl<'a> PartOne<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl Solution for PartOne<'_> {
    type Output = u32;

    fn solve(&self) -> Self::Output {
        self.0
            .lines()
            .filter_map(|line| {
                let mut digits = line.chars().filter(|ch| ch.is_ascii_digit());

                match (digits.next(), digits.last()) {
                    (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
                    (Some(first), None) => Some(format!("{}{}", first, first)),
                    (None, ..) => None,
                }?
                .parse::<Self::Output>()
                .ok()
            })
            .sum()
    }
}

/*
* Part Two
*/
struct PartTwo<'a>(&'a str);

impl<'a> PartTwo<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl Solution for PartTwo<'_> {
    type Output = u32;

    fn solve(&self) -> Self::Output {
        self.0
            .lines()
            .filter_map(|line| {
                let mut digits = DigitParser::new(line);

                match (digits.next(), digits.last()) {
                    (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
                    (Some(first), None) => Some(format!("{}{}", first, first)),
                    (None, ..) => None,
                }?
                .parse::<Self::Output>()
                .ok()
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "
        .trim();

        let expected = 142;
        let solution = PartOne::new(input).solve();

        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "
        .trim();

        let expected = 281;
        let solution = PartTwo::new(input).solve();

        assert_eq!(solution, expected);
    }
}
