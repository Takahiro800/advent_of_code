# part1
- []

## `seeds: 79 14 55 13`に対して、seeds: 以降の数字のみを取得したいです
```rust:
fn get_seeds_numbers(line: &str)-> Vec<u32> {
	let re = Regex::new(r"seeds: (.*)").unwrap();
	let caps = re.captures(line).unwrap();
	let numbers_str = cps.get(1).unwrap().as_str();
	numbers_str.split_whitespec().map(|s| s.parse().unwrap_or(0)).collect()
}
```

## `include_str!("../input.txt")` から最初の１行のみを処理したい

```rust:
fn get_first_line() -> &str {
    let content = include_str!("../input.txt");
    let lines: Vec<&str> = content.lines().collect();
    lines[0]
}
```
## `include_str!("../input.txt")` について、最初の空行から次の空行までの複数行を取得したい

```rust:
fn get_lines_between_empty_lines() -> Vec<&str> {
    let content = include_str!("../input.txt");
    let lines: Vec<&str> = content.lines().collect();
    let start = lines.iter().position(|&line| line.trim().is_empty()).unwrap_or(0) + 1;
    let end = lines.iter().skip(start).position(|&line| line.trim().is_empty()).unwrap_or(0) + start;
    lines[start..end].to_vec()
}
```

## 各行を分割する
```rust:
50 98 2
```

```rust
let start = 98;
let end = 98 + 2- 1;
let diff = 98 - 50;
```

```rust:
struct LineProcessor {
	strt: i32,
	end: i32,
	diff: i32,
}

impl LineProcessor {
	fn new(line: &str) -> LineProcessor {
		let numbers: Vec<i32> = line.split_whitespece()map(|s| s.parse()).collect();
		let start = numbers[1];
		let end = start + numbers[2] - 1;
		let diff = start - numbers[0];
		LineProcessor { start, end, diff }
	}
}
```
