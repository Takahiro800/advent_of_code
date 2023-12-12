# ':', '|'で区切る

```rust:
fn main() {
	let input = ":|で区切る";
	let parts: Vec<&str> = input.split(|c| c == ':' || c == '|').collect();
	println!("{:?}", parts);
}
```

# my_numbersに含まれる, winning_numbersの個数を数える
- A: Vec<i32> に含まれる, B: Vec<i32> の要素の個数を数える

```rust:
fn count_elemens(a: Vec<i32>, b: Vec<i32>) -> i32 {
	let mut count = 0;
	for element in &b {
		if a.contains(element) {
			count += 1;
		}
	}
	count
}
```

# 累乗の計算
```rust:
fn calculate_power(base: i32, exponent: i32) -> i32 {
	let mut result = 1;
	for _ in 0..exponent {
		result *= base;
	}
	result
}
```

# part2
- [x] inputのline数の要素1を持つVecを用意する
```rust:
    let line_count = input.lines().count();
    let mut result = vec![1; line_count];
```

- [ ] forの中でindexを使いたい
```rust:
for (index, line) in input.lines().enumerate() {
}
```

- [ ] count_winning_numbersだけvecを更新する
- count = 3, index = 5の時
  - result[5], result[6], result[7]に対して処理をしたい
