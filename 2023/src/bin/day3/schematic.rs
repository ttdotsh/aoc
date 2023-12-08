use itertools::Itertools;
use std::{convert::Infallible, ops::Range, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct SchematicLine {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

impl FromStr for SchematicLine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.char_indices().peekable();
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        loop {
            match chars.next() {
                Some((idx, ch @ '0'..='9')) => {
                    let rest = chars
                        .peeking_take_while(|(_, char)| char.is_digit(10))
                        .map(|(_, char)| char)
                        .collect::<String>();

                    let value = format!("{}{}", ch, rest);

                    let num = Number {
                        value: value.parse().unwrap(),
                        pos: idx..idx + value.len(),
                    };

                    numbers.push(num);
                }
                Some((_, '.')) => continue,
                Some((idx, _)) => {
                    let sym = Symbol { position: idx };

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
        let Range { start, end } = self.pos;

        let is_adjacent_on_neighboring_line = |s: &Symbol| {
            let in_range = self.pos.contains(&s.position);
            let adjacent_to_start = s.position <= start && start - s.position <= 1;
            let adjacent_to_end = s.position >= end && s.position - end == 0;
            in_range || adjacent_to_start || adjacent_to_end
        };

        let mut has_adj_sym = match above {
            Some(line) => line.symbols.iter().any(is_adjacent_on_neighboring_line),
            None => false,
        };

        has_adj_sym = has_adj_sym
            || curr.symbols.iter().any(|sym| {
                let just_before_start = sym.position < start && start - sym.position == 1;
                let just_after_end = sym.position == end;
                just_before_start || just_after_end
            });

        has_adj_sym = match below {
            Some(line) => has_adj_sym || line.symbols.iter().any(is_adjacent_on_neighboring_line),
            None => has_adj_sym,
        };

        has_adj_sym
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    position: usize,
}
