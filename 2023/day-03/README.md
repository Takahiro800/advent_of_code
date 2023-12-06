# day 03

## 複数行の入力を一文字ずつの２次元の配列にしたい
1. 行ごとに分割
2. 各行を１文字ずつに分解
3. すべての配列を新たな２次元配列に格納する

```rust:
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let two_dimensional_array: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("{:?}", two_dimensional_array);
}
```

## ２次元配列で隣接するいずれかのマスが `#`であるかを確認したい

```rust:
fn check_adjacent(two_dimensional_array: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for direction in directions.iter() {
        let new_i = i as i32 + direction.0;
        let new_j = j as i32 + direction.1;

        if new_i >= 0 && new_j >= 0 && new_i < two_dimensional_array.len() as i32 && new_j < two_dimensional_array[0].len() as i32 {
            if two_dimensional_array[new_i as usize][new_j as usize] == '#' {
                return true;
            }
        }
    }

    false
}

fn main() {
    // ここには上記の2次元配列の作成コードを入れてください。

    for i in 0..two_dimensional_array.len() {
        for j in 0..two_dimensional_array[i].len() {
            if check_adjacent(&two_dimensional_array, i, j) {
                println!("The cell at ({}, {}) is adjacent to '#'", i, j);
            }
        }
    }
}
```

ソートされたintの配列から、連続している箇所に分割したいです。
例えば [3,4,5,10,11,12,15,16]であれば [[3,4,5], [10,11,12], [15,16]]

## markの左上が数字かつ真上は数字でないことをチェックをしたい
```rust:
            if let Some(n) = up_line.get(mark as usize - 1) {
                if n.is_ascii_digit() {
                    result.push(n.to_digit(10).unwrap() as i32);
                }
            }
```

- markの位置を(i, j)とします。
- 左上の位置は(i - 1, j - 1)、真上の位置は(i - 1, j)となります。
- これらの位置の要素をチェックします。ただし、インデックスが負になる場合（つまり、markが配列の端にある場合）はチェックをスキップします。

```rust:
fn check_mark(two_dimensional_array: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
	if i > 0 && j > 0 && two_dimensional_array[i - 1][j - 1].is_ascii_digit() {
		if two_dimensional_array[i - 1][j].is_ascii_digit() == false {
			return true;
		}
	}
	false
}
```

## trueだったら、その数字を取得したい
```rust:
fn check_mark(two_dimensional_array: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
	if i > 0 && j > 0 && two_dimensional_array[i - 1][j - 1].is_ascii_digit() {
		if two_dimensional_array[i - 1][j].is_ascii_digit() == false {
			return Some((i - 1, j - 1));
		}
	}
	None
}
```
=> true をSome(), falseをNoneにしただけ

## 以下の条件を満たすindexを含むVecを返すように修正
- markの左上が数字かつ真上は数字でない
- markの真上が数字である
- markの右上が数字かつ真上は数字でない

```rust:
fn find_indexes(two_dimensional_array: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
	let mut result = Vec::new();

	for i in 1..two_dimensional_array.len() {
		for j in 1..two_dimensional_array[i].len() {
			let left_up = two_dimensional_array[i - 1][j - 1].is_ascii_digit();
			let up = two_dimensional_array[i - 1][j].is_ascii_digit();
			let right_up = j + 1 < two_dimensional_array[i].len() && two_dimensional_array[i - 1][j + 1].is_ascii_digit();

			if (left_up && !up) || up || (right_up && !up) {
				result.push((i, j));
			}
		}
	}

	result
}
```

## Vec<Vec<i32>>から指定した要素を含むVec<i32>のみを返すメソッド
ex) [[1,2,3], [5,6], [10]]に対して、 [2,6]を渡すと[[1,2,3], [5,6]]
