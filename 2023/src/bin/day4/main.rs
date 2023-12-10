use aoc_2023::{show_solutions, Solution};
use std::{
    collections::btree_map::BTreeMap,
    ops::{Bound::*, RangeBounds},
};

fn main() {
    let input = include_str!("../../../puzzle-input/day4.txt");
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
        self.0
            .trim()
            .lines()
            .map(|line| {
                let (_, numbers) = line.trim().split_once(':').unwrap();
                let (winning, actual) = numbers.split_once('|').unwrap();
                let winning = winning.split(' ').collect::<Vec<_>>();

                let matches = actual
                    .split(' ')
                    .filter(|str| !str.is_empty())
                    .filter(|str| winning.contains(str));

                let card_value = matches.fold(0, |acc, _| match acc {
                    0 => 1,
                    v => v * 2,
                });
                card_value
            })
            .sum()
    }
}

struct PartTwo<'a>(&'a str);

impl<'a> PartTwo<'a> {
    fn new(input: &'a str) -> Self {
        Self(input)
    }

    #[allow(dead_code)]
    fn count_scratchcards<R>(cards: &BTreeMap<u32, u32>, range: R) -> u32
    where
        R: RangeBounds<u32>,
    {
        cards
            .range(range)
            .map(|(card_num, num_matches)| {
                let yields = match num_matches {
                    0 => 0,
                    n => {
                        let range = (Excluded(*card_num), Included(*card_num + n));
                        PartTwo::count_scratchcards(cards, range)
                    }
                };
                yields + 1
            })
            .sum()
    }

    #[allow(dead_code)]
    fn count_scratchcards_dynamic(cards: &BTreeMap<u32, u32>) -> u32 {
        let mut yields = BTreeMap::<u32, u32>::new();

        cards
            .iter()
            .rev()
            .fold(0, |total, (card, matched_winners)| {
                let bonus_cards = match matched_winners {
                    0 => 0,
                    n => {
                        let start = card + 1;
                        let end = start + n;
                        (start..end).into_iter().fold(0, |sum, card_num| {
                            if let Some(amt) = yields.get(&card_num) {
                                sum + *amt
                            } else {
                                sum
                            }
                        })
                    }
                };

                let net_worth = bonus_cards + 1;
                yields.insert(*card, net_worth);
                total + net_worth
            })
    }
}

impl Solution for PartTwo<'_> {
    type Output = u32;

    fn solve(&self) -> Self::Output {
        let cards = self
            .0
            .trim()
            .lines()
            .map(|line| {
                let (card, numbers) = line.trim().split_once(':').unwrap();
                let card_num = card
                    .replace("Card", "")
                    .replace(' ', "")
                    .parse::<u32>()
                    .unwrap();
                let (winning, actual) = numbers.split_once('|').unwrap();
                let winning = winning.split(' ').collect::<Vec<_>>();

                let matches = actual
                    .split(' ')
                    .filter(|str| !str.is_empty())
                    .filter(|str| winning.contains(str))
                    .count() as u32;

                (card_num, matches)
            })
            .collect::<BTreeMap<u32, u32>>();

        // PartTwo::count_scratchcards(&cards, ..)
        PartTwo::count_scratchcards_dynamic(&cards)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input_part1() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        let expected = 13;

        let solution = PartOne::new(input).solve();
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_sample_input_part2() {
        let input = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
        .trim();

        let expected = 30;

        let solution = PartTwo::new(input).solve();
        assert_eq!(solution, expected);
    }
}
