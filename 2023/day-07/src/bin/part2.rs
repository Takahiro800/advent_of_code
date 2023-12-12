fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../sample.txt");

    let output = part2(input);
    dbg!(output);

    // let output = part2(input);
    // dbg!(output);
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let mut split_line = line.split_whitespace();
        // TODO: このnextは何？
        let cards = split_line.next().unwrap_or("22222");
        let bet = split_line.next().unwrap().parse::<u32>().unwrap_or(0);

        hands.push(Hand::new(cards, bet));
    }

    sort_hands(&mut hands);

    for hand in &hands {
        println!("{:?}", hand);
    }

    calculate_score(&hands)
}

// 役を作る
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeCard,
    FullHouse,
    FourCard,
    FiveCard,
}
// 各行を役に変換する
#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    rank: HandRank,
    bet: u32,
}

impl Hand {
    fn calculate_hand(&mut self) {
        let mut count_hash = std::collections::HashMap::new();
        let mut cards = self.cards.clone();
        cards.retain(|&c| c != 'J');

        let count_j = self.cards.len() - cards.len();

        for card in cards {
            *count_hash.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<u32> = count_hash.values().cloned().collect();
        counts.sort();

        match counts.last_mut() {
            Some(last) => *last += count_j as u32,
            None => counts.push(count_j as u32),
        }

        self.rank = match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandRank::HighCard,
            [1, 1, 1, 2] => HandRank::OnePair,
            [1, 2, 2] => HandRank::TwoPair,
            [1, 1, 3] => HandRank::ThreeCard,
            [2, 3] => HandRank::FullHouse,
            [1, 4] => HandRank::FourCard,
            [5] => HandRank::FiveCard,
            _ => HandRank::HighCard,
        }
    }

    fn new(cards: &str, bet: u32) -> Hand {
        let mut hand = Hand {
            cards: cards.chars().collect(),
            rank: HandRank::HighCard,
            bet: bet,
        };
        hand.calculate_hand();

        hand
    }
}
// ソートする
const CARD_VALUES: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| match a.rank.cmp(&b.rank) {
        std::cmp::Ordering::Equal => {
            for (card_a, card_b) in a.cards.iter().zip(b.cards.iter()) {
                let value_a = CARD_VALUES.iter().position(|&c| c == *card_a).unwrap();
                let value_b = CARD_VALUES.iter().position(|&c| c == *card_b).unwrap();
                let card_compare = value_a.cmp(&value_b);
                if card_compare != std::cmp::Ordering::Equal {
                    return card_compare;
                }
            }
            std::cmp::Ordering::Equal
        }
        _ => a.rank.cmp(&b.rank),
    });
}

// 得点を集計する
fn calculate_score(hands: &Vec<Hand>) -> u32 {
    let mut score = 0;
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bet * (i as u32 + 1);
    }
    score
}

// fn part2(input: &str) -> i64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(include_str!("../sample.txt"));
        assert_eq!(result, 5905)
    }

    // #[test]
    // fn test_part2() {
    //     let result = part2(include_str!("../sample.txt"));
    //     assert_eq!(result, 71503)
    // }
}
