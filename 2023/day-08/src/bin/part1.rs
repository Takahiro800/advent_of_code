use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../sample.txt");

    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let instructions: Vec<char> = lines[0].chars().collect();
    let ohter_lines: Vec<&str> = lines.iter().skip(2).cloned().collect();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for line in ohter_lines {
        let parts: Vec<&str> = line.split([' ', '=', '(', ',', ')']).collect();
        // println!("{:?}", parts);
        let key = parts[0];
        let node = Node {
            left: parts[4],
            right: parts[6],
        };
        nodes.insert(key, node);
    }
    println!("{:?}", nodes);

    let mut current_map: &str = "AAA";
    let mut index = 0;
    let mut count = 0;
    // println!("{}", current_map);

    while current_map != "ZZZ" {
        let l_or_r = instructions.get(index);
        if let Some(node) = nodes.get(current_map) {
            if l_or_r == Some(&'L') {
                current_map = node.left;
            } else {
                current_map = node.right;
            }
        }
        index = (index + 1) % instructions.len();
        count += 1;
    }
    count
}

fn part2(input: &str) -> u32 {
    0
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../sample.txt"));
        assert_eq!(result, 6)
    }
}
