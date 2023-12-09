mod schematic;

use aoc_2023::{show_solutions, Solution};
use schematic::SchematicLine;

fn main() {
    let input = include_str!("../../../puzzle-input/day3.txt");
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
    type Output = u32;

    fn solve(&self) -> Self::Output {
        let schem_lines = self
            .0
            .lines()
            .map(|line| line.trim().parse::<SchematicLine>().unwrap())
            .collect::<Vec<_>>();

        schem_lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let (above_line, below_line) = match idx {
                    0 => (None, Some(&schem_lines[idx + 1])),
                    i if i == schem_lines.len() - 1 => (Some(&schem_lines[idx - 1]), None),
                    _ => (Some(&schem_lines[idx - 1]), Some(&schem_lines[idx + 1])),
                };

                line.numbers.iter().fold(0, |mut sum, num| {
                    if num.is_schematic_part(above_line, &line, below_line) {
                        sum += num.value;
                    }
                    sum
                })
            })
            .sum()
    }
}

struct PartTwo<'a>(&'a str);

impl<'a> PartTwo<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }
}

impl Solution for PartTwo<'_> {
    type Output = u32;

    fn solve(&self) -> Self::Output {
        let schem_lines = self
            .0
            .lines()
            .map(|line| line.parse::<SchematicLine>().unwrap())
            .collect::<Vec<_>>();

        schem_lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let (before, after) = match idx {
                    0 => (None, Some(&schem_lines[idx + 1])),
                    i if i == schem_lines.len() - 1 => (Some(&schem_lines[idx - 1]), None),
                    _ => (Some(&schem_lines[idx - 1]), Some(&schem_lines[idx + 1])),
                };
                (before, line, after)
            })
            .map(|(before, curr, after)| curr.gears(before, after))
            .map(|gears| gears.iter().fold(0, |sum, gear| sum + gear.ratio()))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part1() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "
        .trim();

        let expected = 4361;
        let solution = PartOne::new(input).solve();

        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.. 
        "
        .trim();

        let expected = 467835;
        let solution = PartTwo::new(input).solve();

        assert_eq!(solution, expected);
    }
}
