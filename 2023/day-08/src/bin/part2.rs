use std::collections::HashMap;

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../sample2.txt");

    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let instructions: Vec<char> = lines[0].chars().collect();
    let ohter_lines: Vec<&str> = lines.iter().skip(2).cloned().collect();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut targets = vec![];

    for line in ohter_lines {
        let parts: Vec<&str> = line.split([' ', '=', '(', ',', ')']).collect();
        let key = parts[0];
        let node = Node {
            left: parts[4],
            right: parts[6],
        };
        nodes.insert(key, node);
        if key.ends_with('A') {
            targets.push(key);
        }
    }

    let mut index = 0;
    let mut count = 0;

    while !all_target_end_with_z(targets.clone()) {
        let l_or_r = instructions.get(index);

        targets = all_target_next(nodes.clone(), targets, *l_or_r.unwrap());

        index = (index + 1) % instructions.len();
        count += 1;
    }
    count
}

fn all_target_end_with_z(nodes: Vec<&str>) -> bool {
    for current in nodes {
        if !current.ends_with('Z') {
            return false;
        }
    }
    true
}

fn all_target_next<'a>(
    nodes: HashMap<&'a str, Node<'a>>,
    targets: Vec<&'a str>,
    l_or_r: char,
) -> Vec<&'a str> {
    let mut result = vec![];
    if l_or_r == 'L' {
        for target in targets {
            if let Some(node) = nodes.get(target) {
                result.push(node.left);
            }
        }
    } else {
        for target in targets {
            if let Some(node) = nodes.get(target) {
                result.push(node.right);
            }
        }
    }
    println!("{:?}", result);
    result
}

#[derive(Debug, Clone)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../sample2.txt"));
        assert_eq!(result, 6)
    }
}
