fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut result = Vec::new();

    for line in input.lines() {
        let first = first_digit(line);
        let last = last_digit(line);

        if let (Some(&first), Some(&last)) = (first.as_ref(), last.as_ref()) {
            let two_digit_number = format!("{}{}", first, last).parse::<i32>().unwrap();
            result.push(two_digit_number);
        }
    }
    result.iter().sum::<i32>().to_string()
}

fn first_digit(line: &str) -> Option<i32> {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut line = line;

    while !line.is_empty() {
        if let Some(digit) = line.chars().next().filter(|c| c.is_ascii_digit()) {
            return Some(digit.to_digit(10).unwrap() as i32);
        } else if let Some(word) = words.iter().find(|&word| line.starts_with(word)) {
            return Some(words.iter().position(|w| w == word).unwrap() as i32);
        } else {
            line = &line[1..];
        }
    }
    None
}

fn last_digit(line: &str) -> Option<i32> {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut line = line;

    while !line.is_empty() {
        if let Some(digit) = line.chars().last().filter(|c| c.is_ascii_digit()) {
            return Some(digit.to_digit(10).unwrap() as i32);
        } else if let Some(word) = words.iter().rfind(|&word| line.ends_with(word)) {
            return Some(words.iter().position(|w| w == word).unwrap() as i32);
        } else {
            line = &line[..line.len() - 1];
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "
            two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        ",
        );
        assert_eq!(result, "281".to_string())
    }
}
