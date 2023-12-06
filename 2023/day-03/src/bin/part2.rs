fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let two_dimensional_array: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let mut result: Vec<i32> = Vec::new();

    // 行ごとにチェック
    for i in 0..two_dimensional_array.len() {
        // *を取得する
        let marks_by_line = marks_by_line(&two_dimensional_array, i);

        // *の周りの数字を取得する
        let calc_nums = get_number_around_marks(&two_dimensional_array, i, marks_by_line);
        result.extend(calc_nums);

        // result.append(&mut check_by_line(&two_dimensional_array, i as i32));
    }

    result.iter().sum::<i32>()
    // result
}

fn get_number_around_marks(
    two_dimensional_array: &Vec<Vec<char>>,
    i: usize,
    marks: Vec<i32>,
) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for mark in marks {
        let mut digits: Vec<i32> = Vec::new();
        // 先に上の行をチェック
        let up_line_index = find_index_up_line(two_dimensional_array, i, mark);
        let mut up_line_digits = disits_by_line(two_dimensional_array, i - 1, up_line_index);
        digits.append(&mut up_line_digits);

        // 同じ行をチェック
        let same_line_index = find_index_same_line(two_dimensional_array, i, mark);
        let mut same_line_digits = disits_by_line(two_dimensional_array, i, same_line_index);
        digits.append(&mut same_line_digits);

        // 下の行をチェック
        let down_line_index = find_index_down_line(two_dimensional_array, i, mark);
        let mut down_line_digits = disits_by_line(two_dimensional_array, i + 1, down_line_index);
        digits.append(&mut down_line_digits);

        if digits.len() == 2 {
            result.push(digits[0] * digits[1]);
        }
    }

    result
}

fn find_index_up_line(two_dimensional_array: &Vec<Vec<char>>, i: usize, mark: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // 先に上の行をチェック
    if i > 0 {
        let up_line = &two_dimensional_array[i - 1];
        let left_up_is_digit = up_line
            .get(mark as usize - 1)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);
        let up_is_digit = up_line
            .get(mark as usize)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);

        let right_up_is_digit = up_line
            .get(mark as usize + 1)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);

        if left_up_is_digit && !up_is_digit {
            result.push(mark - 1);
        }

        if up_is_digit {
            result.push(mark);
        }

        if !up_is_digit && right_up_is_digit {
            result.push(mark + 1);
        }
    }

    result
}

fn find_index_same_line(two_dimensional_array: &Vec<Vec<char>>, i: usize, mark: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // 同じ行をチェック
    let same_line = &two_dimensional_array[i];
    let left_is_digit = same_line
        .get(mark as usize - 1)
        .map(|n| n.is_ascii_digit())
        .unwrap_or(false);

    let right_is_digit = same_line
        .get(mark as usize + 1)
        .map(|n| n.is_ascii_digit())
        .unwrap_or(false);

    if left_is_digit {
        result.push(mark - 1);
    }

    if right_is_digit {
        result.push(mark + 1);
    }

    result
}

fn find_index_down_line(two_dimensional_array: &Vec<Vec<char>>, i: usize, mark: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // 先に上の行をチェック
    if i + 1 < two_dimensional_array.len() {
        let down_line = &two_dimensional_array[i + 1];
        let left_down_is_digit = down_line
            .get(mark as usize - 1)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);
        let down_is_digit = down_line
            .get(mark as usize)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);

        let right_down_is_digit = down_line
            .get(mark as usize + 1)
            .map(|n| n.is_ascii_digit())
            .unwrap_or(false);

        if left_down_is_digit && !down_is_digit {
            result.push(mark - 1);
        }

        if down_is_digit {
            result.push(mark);
        }

        if !down_is_digit && right_down_is_digit {
            result.push(mark + 1);
        }
    }

    result
}

fn disits_by_line(
    two_dimensional_array: &Vec<Vec<char>>,
    i: usize,
    index_array: Vec<i32>,
) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut line_digits: Vec<i32> = Vec::new();

    for (index, value) in two_dimensional_array[i as usize].iter().enumerate() {
        if value.is_ascii_digit() {
            line_digits.push(index as i32);
        }
    }

    // 数値のみを取り出す
    let didit_array = split(line_digits);

    let filtered_disit_vec = filter_vecs(didit_array, index_array);

    // filterd_disit_veを使って、数字をと取り出す
    for disit_vec in filtered_disit_vec {
        let digits: Vec<&char> = disit_vec
            .iter()
            .filter_map(|&index| two_dimensional_array[i].get(index as usize))
            .collect();

        if let Ok(n) = chars2i32(digits) {
            result.push(n);
        }
    }

    result
}

fn filter_vecs(vecs: Vec<Vec<i32>>, elements: Vec<i32>) -> Vec<Vec<i32>> {
    vecs.into_iter()
        .filter(|vec| vec.iter().any(|item| elements.contains(item)))
        .collect()
}

fn marks_by_line(two_dimensional_array: &Vec<Vec<char>>, i: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for (index, value) in two_dimensional_array[i].iter().enumerate() {
        if value == &'*' {
            result.push(index as i32);
        }
    }

    result
}

fn check_by_line(two_dimensional_array: &Vec<Vec<char>>, i: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // その行において、数字であるマスのindexを配列にいれるAA
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
            if new_i < two_dimensional_array.len()
                && new_j < two_dimensional_array[0].len()
                && !signs.contains(&two_dimensional_array[new_i][new_j])
            {
                return true;
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
    fn test_part2() {
        let input = include_str!("../sample.txt");
        let result = part2(input);
        assert_eq!(result, 467835)
    }
}
