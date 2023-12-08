use std::{fmt::Display, str::FromStr};

pub struct DigitParser<'p> {
    input: &'p str,
    position: usize,
}

impl<'p> DigitParser<'p> {
    pub fn new<'input: 'p>(input: &'input str) -> Self {
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

            let slice = &self.input[self.position..end];
            if let Ok(digit) = slice.parse() {
                match slice.len() {
                    1 => self.position += 1,
                    _ => self.position = end - 1,
                };
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

#[derive(Debug, PartialEq, Eq)]
pub enum Digit {
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
}
