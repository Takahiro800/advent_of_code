# 方針
- [ ] 入力を取得
- [ ] １行目を束縛
  - instructions
- [ ] ３行目以降を束縛
- [ ] HashMapに保存
  - [ ] Nodeにする


# ３行目以降を束縛

```rust:
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}
```

```bash:
missing lifetime specifier expected named lifetime parameter
```

- [] skip
  - [3]だとAAAが取得できない

# lcm で解決する
- 証明ができてないのできちんと確認する
