#![allow(dead_code)]

use std::{fmt::Display, str::FromStr};

fn main() {
    let path = include_str!("../../input.txt");
    let sum = solve(path);
    println!("{}", sum);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| parse_calibration_values(line))
        .sum()
}

fn parse_calibration_values(input: &str) -> Option<u32> {
    let mut digits = DigitParser::new(input);

    match (digits.next(), digits.last()) {
        (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
        (Some(first), None) => Some(format!("{}{}", first, first)),
        (None, ..) => None,
    }?
    .parse()
    .ok()
}

#[derive(Debug, PartialEq, Eq)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Digit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Digit::*;

        match s {
            "1" | "one" => Ok(One),
            "2" | "two" => Ok(Two),
            "3" | "three" => Ok(Three),
            "4" | "four" => Ok(Four),
            "5" | "five" => Ok(Five),
            "6" | "six" => Ok(Six),
            "7" | "seven" => Ok(Seven),
            "8" | "eight" => Ok(Eight),
            "9" | "nine" => Ok(Nine),
            _ => Err("no digit found"),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Digit::*;

        let digit = match self {
            One => "1",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
        };
        write!(f, "{}", digit)
    }
}

struct DigitParser<'p> {
    input: &'p str,
    position: usize,
}

impl<'p> DigitParser<'p> {
    fn new<'input: 'p>(input: &'input str) -> Self {
        DigitParser { input, position: 0 }
    }
}

impl Iterator for DigitParser<'_> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut end = self.position + 1;

        loop {
            if end > self.input.len() {
                break None;
            }

            if let Ok(digit) = self.input[self.position..end].parse() {
                self.position += 1;
                break Some(digit);
            }

            if end - self.position > 5 || end == self.input.len() {
                self.position += 1;
                end = self.position;
            } else {
                end += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digit_parser() {
        let input = "two1nine";
        let mut dp = DigitParser::new(input);

        assert_eq!(dp.next(), Some(Digit::Two));
        assert_eq!(dp.next(), Some(Digit::One));
        assert_eq!(dp.next(), Some(Digit::Nine));

        let input = "eightwothree";
        let mut dp = DigitParser::new(input);

        assert_eq!(dp.next(), Some(Digit::Eight));
        assert_eq!(dp.next(), Some(Digit::Two));
        assert_eq!(dp.next(), Some(Digit::Three));

        let input = "xtwone3four";
        let mut dp = DigitParser::new(input);

        assert_eq!(dp.next(), Some(Digit::Two));
        assert_eq!(dp.next(), Some(Digit::One));
        assert_eq!(dp.next(), Some(Digit::Three));
        assert_eq!(dp.next(), Some(Digit::Four));

        let input = "zoneight234";
        let mut dp = DigitParser::new(input);

        assert_eq!(dp.next(), Some(Digit::One));
        assert_eq!(dp.next(), Some(Digit::Eight));
        assert_eq!(dp.next(), Some(Digit::Two));
        assert_eq!(dp.next(), Some(Digit::Three));
        assert_eq!(dp.next(), Some(Digit::Four));
    }

    #[test]
    fn test_parse_sample_part2_input() {
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

        let expected = [29, 83, 13, 24, 42, 14, 76];

        for (i, line) in input.lines().enumerate() {
            let calibration_value = parse_calibration_values(line).unwrap();
            let expected = expected[i];
            assert_eq!(calibration_value, expected)
        }
    }
}
