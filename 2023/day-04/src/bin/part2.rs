use std::result;

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let line_count = input.lines().count();
    let mut result = vec![1; line_count];

    for (index, line) in input.lines().enumerate() {
        let card_count = result[index];
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
        record_card(&mut result, index, count_winning_numbers);
    }

    result.iter().sum()
}

// 取得したカードを記録する
fn record_card(result: &mut Vec<i32>, current_index: usize, winning_count: i32) {
    if winning_count < 0 || current_index >= result.len() {
        return;
    }
    for i in 1..=winning_count {
        if current_index + i as usize >= result.len() {
            break;
        }
        result[current_index + i as usize] += result[current_index];
    }
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
        let result = part2(include_str!("../sample.txt"));
        assert_eq!(result, 30)
    }
}
