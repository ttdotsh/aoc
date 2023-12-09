use super::maybe_chain::ChainOption;
use itertools::Itertools;
use std::{convert::Infallible, iter::once, ops::Range, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct SchematicLine {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

impl SchematicLine {
    pub fn gears(&self, before: Option<&Self>, after: Option<&Self>) -> Vec<Gear> {
        self.symbols
            .iter()
            .filter(|sym| sym.ch == '*')
            .filter_map(|sym| {
                let curr = self.numbers.iter();
                let before = before.map(|line| line.numbers.iter());
                let after = after.map(|line| line.numbers.iter());

                let adjacent = curr
                    .maybe_chain(before)
                    .maybe_chain(after)
                    .filter(|num| sym.is_adjacent_to(num))
                    .collect_tuple();

                match adjacent {
                    Some((first, second)) => Some(Gear(first.value, second.value)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for SchematicLine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().char_indices().peekable();
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        loop {
            match chars.next() {
                Some((idx, ch @ '0'..='9')) => {
                    let rest = chars
                        .peeking_take_while(|(_, char)| char.is_ascii_digit())
                        .map(|(_, char)| char);
                    let value = once(ch).chain(rest).collect::<String>();

                    let num = Number {
                        value: value.parse().unwrap(),
                        pos: idx..idx + value.len(),
                    };

                    numbers.push(num);
                }
                Some((_, '.')) => continue,
                Some((idx, ch)) => {
                    let sym = Symbol { ch, pos: idx };

                    symbols.push(sym);
                }
                None => break,
            };
        }

        Ok(SchematicLine { numbers, symbols })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    pub value: u32,
    pub pos: Range<usize>,
}

impl Number {
    pub fn is_schematic_part(
        &self,
        above: Option<&SchematicLine>,
        curr: &SchematicLine,
        below: Option<&SchematicLine>,
    ) -> bool {
        let has_adjacent_above = match above {
            Some(line) => line.symbols.iter().any(|sym| sym.is_adjacent_to(self)),
            None => false,
        };

        let has_adjacent_curr = curr.symbols.iter().any(|sym| sym.is_adjacent_to(self));

        let has_adjacent_below = match below {
            Some(line) => line.symbols.iter().any(|sym| sym.is_adjacent_to(self)),
            None => false,
        };

        has_adjacent_above || has_adjacent_curr || has_adjacent_below
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    ch: char,
    pos: usize,
}

impl Symbol {
    fn is_adjacent_to(&self, num: &Number) -> bool {
        let Range { start, end } = num.pos;

        let in_range = num.pos.contains(&self.pos);
        let adjacent_to_start = self.pos <= start && start - self.pos <= 1;
        let adjacent_to_end = self.pos == end;

        in_range || adjacent_to_start || adjacent_to_end
    }
}

pub struct Gear(u32, u32);

impl Gear {
    pub fn ratio(&self) -> u32 {
        self.0 * self.1
    }
}
