use ::regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../sample.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let mut numbers = get_seeds_numbers(lines[0]);
    let white_lines_index = get_white_lines(&lines);

    println!("seed-to-soil");
    let seed_processors = get_processors(&lines, white_lines_index[0] + 2);
    numbers = translate(numbers, seed_processors);

    println!("soil-to-fertilizer");
    let soil_processors = get_processors(&lines, white_lines_index[1] + 2);
    numbers = translate(numbers, soil_processors);

    println!("fertilizer-to-water");
    let fertilizer_processors = get_processors(&lines, white_lines_index[2] + 2);
    numbers = translate(numbers, fertilizer_processors);

    println!("water-to-light");
    let water_processors = get_processors(&lines, white_lines_index[3] + 2);
    numbers = translate(numbers, water_processors);

    println!("light-to-temperature");
    let light_processors = get_processors(&lines, white_lines_index[4] + 2);
    numbers = translate(numbers, light_processors);

    println!("temperature-to-humidity");
    let temperature_processors = get_processors(&lines, white_lines_index[5] + 2);
    numbers = translate(numbers, temperature_processors);

    println!("humidity-to-location");
    let humidity_processors = get_processors(&lines, white_lines_index[6] + 2);
    numbers = translate(numbers, humidity_processors);

    *numbers.iter().min().unwrap_or(&0)
}

fn translate(numbers: Vec<i64>, processors: Vec<LineProcessor>) -> Vec<i64> {
    let mut numbers = numbers
        .iter()
        .map(|num| (*num, false))
        .collect::<Vec<(i64, bool)>>();

    for proseccor in processors {
        numbers = numbers
            .into_iter()
            .map(|(num, processed)| {
                if !processed && num >= proseccor.start && num <= proseccor.end {
                    (num + proseccor.diff, true)
                } else {
                    (num, processed)
                }
            })
            .collect();
    }

    let numbers: Vec<i64> = numbers.into_iter().map(|(num, _)| num).collect();
    // println!("{:?}", numbers);
    numbers
}

fn get_seeds_numbers(line: &str) -> Vec<i64> {
    let re = Regex::new(r"seeds: (.*)").unwrap();
    let caps = re.captures(line).unwrap();
    let seeds: Vec<i64> = caps[1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut numbers = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        numbers.extend(seeds[i]..=seeds[i] + seeds[i + 1]);
    }

    numbers
}

// 空行を探す
fn get_white_lines(lines: &Vec<&str>) -> Vec<usize> {
    let mut result = Vec::new();
    lines.iter().enumerate().for_each(|(i, line)| {
        if line.is_empty() {
            result.push(i);
        }
    });

    result
}

// 空行から次の空行までの行を取得して、LineProcessorに渡す
fn get_processors(lines: &Vec<&str>, start: usize) -> Vec<LineProcessor> {
    let mut result = Vec::new();

    for i in start..lines.len() {
        if lines[i].is_empty() {
            break;
        }
        let processor = LineProcessor::new(lines[i]);
        result.push(processor);
    }
    result
}

struct LineProcessor {
    start: i64,
    end: i64,
    diff: i64,
}

impl LineProcessor {
    fn new(line: &str) -> LineProcessor {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        let start = numbers[1];
        let end = start + numbers[2] - 1;
        let diff = numbers[0] - start;
        // println!("{} {} {}", start, end, diff);

        LineProcessor { start, end, diff }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../sample.txt"));
        assert_eq!(result, 46)
    }
}
