use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut result = Vec::new();

    for line in input.lines() {
        let set_volume = get_set_volume(line);
        result.push(set_volume);
    }
    result.iter().sum::<i32>()
}

fn get_max_count(line: &str, color: &str) -> Option<i32> {
    let re = Regex::new(&format!(r"(\d+) {}", color)).unwrap();
    line.split(';')
        .filter_map(|part| {
            re.captures(part)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().parse::<i32>().unwrap())
        })
        .max()
}

fn get_set_volume(line: &str) -> i32 {
    let red_count = get_max_count(line, "red").unwrap_or(1);
    let green_count = get_max_count(line, "green").unwrap_or(1);
    let blue_count = get_max_count(line, "blue").unwrap_or(1);

    red_count * green_count * blue_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286)
    }

    #[test]
    fn test_get_red() {
        let result = get_max_count(
            " Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "red",
        );
        assert_eq!(result, Some(20))
    }

    #[test]

    fn test_get_set_volume() {
        let result = get_set_volume("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 36)
    }
}
