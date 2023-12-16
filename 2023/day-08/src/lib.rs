pub trait Solve {
    type Answer1: std::fmt::Display;
    type Answer2: std::fmt::Display;

    fn new() -> Self;
    fn part1(&self) -> Self::Answer1;
    fn part2(&self) -> Self::Answer2;
}
