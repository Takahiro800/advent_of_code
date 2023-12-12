```
Time:        59     68     82     74
Distance:   543   1020   1664   1022
```

- [ ] 入力を受け取る
- [ ] time [59, 68, 82, 74], distances: [543, 1020, 1664, 1022]
- [ ] calc_distance(push_time) = push_time*(time- push_time+ 1)
- [ ] search_min_time
- [ ] search_max_time
- [ ] ans = max_time - min_time + 1

# 入力を受け取る
-  time [59, 68, 82, 74], distances: [543, 1020, 1664, 1022]

# calc_distance(push_time) = push_time*(time- push_time+ 1)
# search_min_time
1..=zにおいて、calc_distanceの結果が与えられた値を超える最小の値を求めたいです。
binary_searchを使ってください

- (left + right) /2 と `left + (right - left)/2`の違い
- lower binary search
# search_max_time
# ans = max_time - min_time + 1
