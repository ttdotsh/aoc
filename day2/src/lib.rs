use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub reveals: Vec<Reveal>,
}

impl Game {
    pub fn is_possible(&self, colors: &(u32, u32, u32)) -> bool {
        self.reveals.iter().all(|reveal| {
            let (red, green, blue) = colors;

            let mut possible = if let Some(r) = reveal.red {
                r <= *red
            } else {
                true
            };

            if let Some(g) = reveal.green {
                possible = possible && g <= *green;
            }

            if let Some(b) = reveal.blue {
                possible = possible && b <= *blue;
            }

            possible
        })
    }

    pub fn min_cubes_required(&self) -> (u32, u32, u32) {
        self.reveals.iter().fold((0, 0, 0), |mut cubes, reveal| {
            if let Some(red) = reveal.red {
                if red > cubes.0 {
                    cubes.0 = red;
                }
            }

            if let Some(green) = reveal.green {
                if green > cubes.1 {
                    cubes.1 = green;
                }
            }

            if let Some(blue) = reveal.blue {
                if blue > cubes.2 {
                    cubes.2 = blue;
                }
            }

            cubes
        })
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, summary) = match s.split_once(':') {
            Some(split) => Ok(split),
            None => Err("did not find a ':' to split on"),
        }?;

        let id = label
            .replace("Game ", "")
            .parse()
            .map_err(|_| "unable to parse game number")?;

        let reveals = summary
            .split(';')
            .map(|str| str.parse())
            .collect::<Result<Vec<Reveal>, &'static str>>()?;

        Ok(Game { id, reveals })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reveal {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl FromStr for Reveal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (red, green, blue) = s.split(',').try_fold((None, None, None), |mut colors, c| {
            match c.trim().split_once(' ') {
                Some((num, "red")) => colors.0 = num.parse().ok(),
                Some((num, "green")) => colors.1 = num.parse().ok(),
                Some((num, "blue")) => colors.2 = num.parse().ok(),
                _ => return Err("unable to parse colors"),
            };
            Ok(colors)
        })?;

        Ok(Reveal { red, green, blue })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected = Game {
            id: 1,
            reveals: vec![
                Reveal {
                    red: Some(4),
                    green: None,
                    blue: Some(3),
                },
                Reveal {
                    red: Some(1),
                    green: Some(2),
                    blue: Some(6),
                },
                Reveal {
                    red: None,
                    green: Some(2),
                    blue: None,
                },
            ],
        };

        let game = input.parse::<Game>().unwrap();
        assert_eq!(game, expected);
    }
}
