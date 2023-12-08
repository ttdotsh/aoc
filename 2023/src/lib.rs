use std::fmt::Display;

pub trait Solution {
    type Output: Display;

    fn solve(&self) -> Self::Output;
}

pub fn show_solutions<P1, P2>(part1: P1, part2: P2)
where
    P1: Solution,
    P2: Solution,
{
    println!("Part 1: {}", part1.solve());
    println!("Part 2: {}", part2.solve());
}
