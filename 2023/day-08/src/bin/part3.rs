use day_08::Solve;
use itertools::Itertools;
use std::collections::HashMap;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

struct Solution {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

impl Solution {
    fn num_steps(&self) -> u64 {
        self.network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(String::as_ref)
            .map(|s| self.find_cycle(s))
            .fold(1, lcm)
    }

    fn find_cycle(&self, start: &str) -> u64 {
        let (mut current, mut hash_map) = (start, HashMap::new());
        let mut instructions = self.instructions.chars().enumerate().cycle();

        for i in 1_u64.. {
            let (index, instruction) = instructions.next().expect("No instructions");
            let (l, r) = &self.network[current];

            current = match instruction {
                'L' => l,
                'R' => r,
                _ => unreachable!(),
            };

            if current == "ZZZ" {
                println!("cycle-i: {}", i);
                return i;
            }
            if let Some(p) = hash_map.get(&(index, current)) {
                println!("cycle-i-p: {}", i - p);
                return i - p;
            }
            hash_map.insert((index, current), i);
        }
        unreachable!();
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new() -> Self {
        let input = include_str!("../input.txt");
        // let input = include_str!("../sample.txt");
        let lines: Vec<&str> = input.lines().collect();

        Self {
            instructions: lines[0].chars().collect(),
            network: lines[2..]
                .iter()
                .map(|line| {
                    line.split_once(" = ")
                        .map(|(label, nodes)| {
                            (
                                label.into(),
                                nodes
                                    .trim_matches(|c| c == '(' || c == ')')
                                    .split(", ")
                                    .map(String::from)
                                    .collect_tuple()
                                    .expect("invalid nodes"),
                            )
                        })
                        .expect("invalide line")
                })
                .collect(),
        }
    }

    fn part1(&self) -> Self::Answer1 {
        self.num_steps()
    }
    fn part2(&self) -> Self::Answer2 {
        self.num_steps()
    }
}

fn main() {
    let solution = Solution::new();
    println!("Part 2: {}", solution.part2());
}
