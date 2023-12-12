fn main() {
    let input = include_str!("../input.txt");

    let output = part1(input);
    dbg!(output);

    let output = part2(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let times = get_array(lines[0]);
    let distances = get_array(lines[1]);

    let mut result = Vec::new();

    for (time, distance) in times.iter().zip(distances.iter()) {
        println!("time: {}, distance: {}", time, distance);
        let min_time = find_min_time(calc_distance, *distance, *time);
        let max_time = time - min_time;
        result.push(max_time - min_time + 1);
    }
    println!("{:?}", result);

    result.iter().product()
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let time = line_to_i64(lines[0]);
    let distance = line_to_i64(lines[1]);

    let min_time = find_min_time(calc_distance, distance, time);
    time - 2 * min_time + 1
}

fn line_to_i64(line: &str) -> i64 {
    line.split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .concat()
        .parse()
        .unwrap()
}

fn get_array(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calc_distance(limit_time: i64, push_time: i64) -> i64 {
    push_time * (limit_time - push_time)
}

fn find_min_time(calc_distance: fn(i64, i64) -> i64, record: i64, limit_time: i64) -> i64 {
    let mut left = 1;
    let mut right = limit_time;

    while left < right {
        let mid = left + (right - left) / 2;
        let distance = calc_distance(limit_time, mid);

        println!(
            "min!!!!left: {}, right: {}, mid: {}, dis:{}, record: {}",
            left, right, mid, distance, record
        );

        if distance <= record {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(include_str!("../sample.txt"));
        assert_eq!(result, 288)
    }

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../sample.txt"));
        assert_eq!(result, 71503)
    }
}
