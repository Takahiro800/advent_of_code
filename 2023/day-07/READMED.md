# cardsの要素にも強さを設ける
- cards: Vec[char]は["A", "K", "Q", "J", "T","9", "8", ..., "2"]の要素から構成されています
- 左から強い順に並んでいます。
- a: Hand, b: Handを比較するときは
  1. rankの強さ
  2. 1が等しい場合にcards[0]の強さ
  3. 2が等しい場合にcards[1]の強さ
  4. ...
  5. cards[4]の強さ

という比較をしたいです
- この強さはどのように表現すれば良いですか？

countsが
5を含んでいれば FiveCard
4を含んでいれば FourCard
3と2を含んでいれば FullHouse,
3を含んでいれば ThreeCard
2を二つ含んでいればTwoPair
1を二つ含んでいればOnePairにしたいです
