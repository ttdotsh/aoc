fn main() {
    let path = include_str!("../../input.txt");
    let sum = solve(path);
    println!("{}", sum);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| parse_calibration_values(l))
        .sum()
}

fn parse_calibration_values(input: &str) -> Option<u32> {
    let mut digits = input.chars().filter(|ch| ch.is_digit(10));

    match (digits.next(), digits.last()) {
        (Some(first), Some(last)) => Some(format!("{}{}", first, last)),
        (Some(first), None) => Some(format!("{}{}", first, first)),
        (None, ..) => None,
    }?
    .parse()
    .ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_sample_input() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "
        .trim();

        let expected = [12, 38, 15, 77];

        for (i, line) in input.lines().enumerate() {
            let calibration_value = parse_calibration_values(line).unwrap();
            let expected = expected[i];
            assert_eq!(calibration_value, expected)
        }
    }
}
