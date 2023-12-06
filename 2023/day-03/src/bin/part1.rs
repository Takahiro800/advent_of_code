fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

// fn part1(input: &str) -> Vec<i32> {
fn part1(input: &str) -> i32 {
    let two_dimensional_array: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let mut result: Vec<i32> = Vec::new();

    // 行ごとにチェック
    for i in 0..two_dimensional_array.len() {
        result.append(&mut check_by_line(&two_dimensional_array, i as i32));
    }

    result.iter().sum::<i32>()
    // result
}

fn check_by_line(two_dimensional_array: &Vec<Vec<char>>, i: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // その行において、数字であるマスのindexを配列にいれる
    let mut index_array: Vec<i32> = Vec::new();
    for (index, value) in two_dimensional_array[i as usize].iter().enumerate() {
        if value.is_ascii_digit() {
            index_array.push(index as i32);
        }
    }

    // 数値のみを取り出す
    let didit_array = split(index_array);

    didit_array.iter().for_each(|array| {
        let start_j = array[0];
        let end_j = array[array.len() - 1];
        if check_adjacent(
            two_dimensional_array,
            i as usize,
            start_j.try_into().unwrap(),
            end_j.try_into().unwrap(),
        ) {
            let digits: Vec<&char> = array
                .iter()
                .filter_map(|&index| two_dimensional_array[i as usize].get(index as usize))
                .collect();

            if let Ok(n) = chars2i32(digits) {
                result.push(n);
            }
        }
    });

    result
}

fn chars2i32(digits: Vec<&char>) -> Result<i32, std::num::ParseIntError> {
    let string_digit: String = digits.iter().map(|c| c.to_string()).collect();

    string_digit.parse::<i32>()
}

fn check_adjacent(
    two_dimensional_array: &Vec<Vec<char>>,
    i: usize,
    start_j: usize,
    end_j: usize,
) -> bool {
    let i_range = if i > 0 { i - 1 } else { i }..=if i < usize::MAX - 1 { i + 1 } else { i };
    let start_j = if start_j > 0 { start_j - 1 } else { start_j };
    let signs = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

    for new_i in i_range {
        let j_range = if end_j < usize::MAX - 1 {
            start_j..=end_j + 1
        } else {
            start_j..=end_j
        };
        for new_j in j_range {
            if new_i < two_dimensional_array.len() && new_j < two_dimensional_array[0].len() {
                if !signs.contains(&two_dimensional_array[new_i][new_j]) {
                    return true;
                }
            }
        }
    }

    false
}

fn split(array: Vec<i32>) -> Vec<Vec<i32>> {
    if array.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<Vec<i32>> = vec![vec![array[0]]];

    for i in 1..array.len() {
        if array[i] - array[i - 1] == 1 {
            result.last_mut().unwrap().push(array[i]);
        } else {
            result.push(vec![array[i]]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../sample.txt");
        let result = part1(input);
        assert_eq!(result, 4361)
    }
}
