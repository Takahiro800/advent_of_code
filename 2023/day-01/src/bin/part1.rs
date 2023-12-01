fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // "142".to_string()

    let mut result = String::new();
    for line in input.lines() {
        println!("{}", line);

        result.push_str(&line);
        result.push('\n');
    }
    result.to_string()
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
        assert_eq!(
            result,
            "
        1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        \n"
            .to_string()
        )
    }
}
