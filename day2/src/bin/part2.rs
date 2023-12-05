use day2::Game;

fn main() {
    let input = include_str!("../../input.txt");
    let sum = solve(input);
    println!("{}", sum);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = line.trim().parse::<Game>().ok()?;
            let (red, green, blue) = game.min_cubes_required();
            let power = red * green * blue;
            Some(power)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2_sample() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let sum = solve(input.trim());
        assert_eq!(sum, 2286);
    }
}
