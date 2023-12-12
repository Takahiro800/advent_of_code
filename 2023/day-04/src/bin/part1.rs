fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(':').flat_map(|x| x.split('|')).collect();

        // TODO: Vec<i32>に変換する関数を作成する
        let winning_numbers = split_line[1]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect();
        let my_numbers = split_line[2]
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect::<Vec<_>>();

        let count_winning_numbers = count_elemens(my_numbers, winning_numbers);
        let point = calculate_power(2, count_winning_numbers);
        result.push(point);
    }

    result.iter().sum()
}

//  my_numbersに含まれる, winning_numbersの個数を数える
fn count_elemens(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut count = 0;
    for element in &b {
        if a.contains(element) {
            count += 1;
        }
    }
    count
}

//  累乗の計算
fn calculate_power(base: i32, exponent: i32) -> i32 {
    let mut result = 1;
    if exponent == 0 {
        return 0;
    }

    for _ in 0..exponent - 1 {
        result *= base;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part1(include_str!("../sample.txt"));
        assert_eq!(result, 13)
    }

    #[test]

    fn test_count_elemens() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![1, 2, 3];
        let result = count_elemens(a, b);
        assert_eq!(result, 3)
    }
}
