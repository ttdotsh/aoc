mod game;

use aoc_2023::{show_solutions, Solution};
use game::Game;

fn main() {
    let input = include_str!("../../../puzzle-input/day2.txt");

    let part1_colors = (12, 13, 14);
    let part1 = PartOne::new(input, part1_colors);

    let part2 = PartTwo::new(input);

    show_solutions(part1, part2);
}

/*
* Part One
*/
struct PartOne<'a> {
    input: &'a str,
    colors: (u32, u32, u32),
}

impl<'a> PartOne<'a> {
    fn new(input: &'a str, colors: (u32, u32, u32)) -> Self {
        Self { input, colors }
    }
}

impl Solution for PartOne<'_> {
    type Output = u32;

    fn solve(&self) -> Self::Output {
        self.input
            .trim()
            .lines()
            .filter_map(|line| {
                let game = line.trim().parse::<Game>().ok()?;
                match game.is_possible(&self.colors) {
                    true => Some(game.id),
                    false => None,
                }
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
            .trim()
            .lines()
            .filter_map(|line| {
                let game = line.trim().parse::<Game>().ok()?;
                let (red, green, blue) = game.min_cubes_required();
                let power = red * green * blue;
                Some(power)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part1() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let colors = (12, 13, 14);
        let expected = 8;
        let solution = PartOne::new(input, colors).solve();
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let expected = 2286;
        let solution = PartTwo::new(input).solve();
        assert_eq!(solution, expected);
    }
}
