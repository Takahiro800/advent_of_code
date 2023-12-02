fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result = Vec::new();
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|ch| ch.is_ascii_digit()).collect();
        if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
            let two_digit_number = format!("{}{}", first, last).parse::<i32>().unwrap();
            result.push(two_digit_number);
        }
    }
    result.iter().sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "
        1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        ",
        );
        // assert_eq!(result, "142".to_string())
        assert_eq!(result, "142".to_string())
    }
}
