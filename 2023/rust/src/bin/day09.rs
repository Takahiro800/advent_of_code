use rust::Solve;

fn next_value(histroy: Vec<i32>) -> i32 {
    let mut stack = histroy;
    let mut sum = *stack.last().unwrap_or(&0);

    // stackの最後の要素が０になるまで繰り返す
    while !stack.iter().all(|&x| x == 0) {
        stack = stack.windows(2).map(|w| w[1] - w[0]).collect();
        sum += stack.last().unwrap_or(&0);
    }

    sum
}

struct Solution {
    histories: Vec<Vec<i32>>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            histories: lines
                .iter()
                .filter(|line| !line.trim().is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect()
                })
                .collect(),
        }
    }

    fn part1(&self) -> Self::Answer1 {
        self.histories.iter().map(|h| next_value(h.clone())).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.histories
            .iter()
            .map(|h| {
                let mut rev_h = h.clone();
                rev_h.reverse();
                next_value(rev_h)
                // next_value(h.clone(), false)
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("../../input/09.txt");
    let solution = Solution::new(input);
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test09_part1() {
        let input = include_str!("../../sample/09.txt");
        assert_eq!(Solution::new(input).part1(), 114)
    }

    #[test]
    fn test09_part2() {
        let input = include_str!("../../sample/09.txt");
        assert_eq!(Solution::new(input).part2(), 2)
    }
}
