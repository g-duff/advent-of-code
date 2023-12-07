use std::{collections, fs, str};

fn main() {
    let input = &fs::read_to_string("./data/07.input").unwrap();

    let hands: Vec<Hand> = input.lines().filter_map(|l| l.parse().ok()).collect();

    // let pt1_answer = solve_pt1(hands);
    // println!("{pt1_answer}");

    let pt2_answer = solve_pt2(hands);
    println!("{pt2_answer}");
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: i64,
}

fn solve_pt1(hands: Vec<Hand>) -> i64 {
    let mut ranked_bids: Vec<RankedBid> = hands.into_iter().map(|h| h.to_ranked_bid()).collect();

    ranked_bids.sort_by(|a, b| a.rank.cmp(&b.rank));

    ranked_bids
        .iter()
        .enumerate()
        .map(|(i, b)| (i as i64 + 1) * b.bid)
        .sum()
}

fn solve_pt2(hands: Vec<Hand>) -> i64 {
    let mut ranked_bids: Vec<RankedBid> =
        hands.into_iter().map(|h| h.to_ranked_bid_pt2()).collect();

    ranked_bids.sort_by(|a, b| a.rank.cmp(&b.rank));

    ranked_bids
        .iter()
        .enumerate()
        .map(|(i, b)| (i as i64 + 1) * b.bid)
        .sum()
}

impl Hand {
    fn to_ranked_bid(self) -> RankedBid {
        let small_rank: i64 = self
            .cards
            .chars()
            .map(|c| {
                let d = match c {
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => 0,
                };
                d
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (i, a)| acc + (100_i64.pow(i as u32) * a));

        let mut card_counts = collections::HashMap::new();

        self.cards.chars().for_each(|c| {
            let super_account_amount = card_counts.entry(c).or_insert(0);
            *super_account_amount += 1
        });

        let mut hand_possibilities: Vec<i64> = card_counts.into_values().collect();
        hand_possibilities.sort();
        hand_possibilities.reverse();

        let mut big_score = 0;
        if hand_possibilities == vec![5] {
            big_score = 60000000000
        } else if hand_possibilities == vec![4, 1] {
            big_score = 50000000000
        } else if hand_possibilities == vec![3, 2] {
            big_score = 40000000000
        } else if hand_possibilities == vec![3, 1, 1] {
            big_score = 30000000000
        } else if hand_possibilities == vec![2, 2, 1] {
            big_score = 20000000000
        } else if hand_possibilities == vec![2, 1, 1, 1] {
            big_score = 10000000000
        }

        RankedBid {
            bid: self.bid,
            rank: big_score + small_rank,
        }
    }

    fn to_ranked_bid_pt2(self) -> RankedBid {
        let small_rank: i64 = self
            .cards
            .chars()
            .map(|c| {
                let d = match c {
                    'J' => 0,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'T' => 10,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => 0,
                };
                d
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (i, a)| acc + (100_i64.pow(i as u32) * a));

        let mut card_counts = collections::HashMap::new();
        let mut j = 0;

        self.cards.chars().for_each(|c| {
            if c == 'J' {
                j += 1
            } else {
                let super_account_amount = card_counts.entry(c).or_insert(0);
                *super_account_amount += 1
            }
        });

        let mut hand_possibilities: Vec<i64> = card_counts.into_values().collect();
        hand_possibilities.sort();
        hand_possibilities.reverse();

        if j == 5 {
            hand_possibilities = vec![5];
        } else {
            hand_possibilities[0] += j;
        }

        let mut big_score = 0;

        if hand_possibilities == vec![5] {
            big_score = 60000000000
        } else if hand_possibilities == vec![4, 1] {
            big_score = 50000000000
        } else if hand_possibilities == vec![3, 2] {
            big_score = 40000000000
        } else if hand_possibilities == vec![3, 1, 1] {
            big_score = 30000000000
        } else if hand_possibilities == vec![2, 2, 1] {
            big_score = 20000000000
        } else if hand_possibilities == vec![2, 1, 1, 1] {
            big_score = 10000000000
        }

        RankedBid {
            bid: self.bid,
            rank: big_score + small_rank,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RankedBid {
    bid: i64,
    rank: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl str::FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let (cards, bid) = s.split_once(" ").unwrap();

        Ok(Hand {
            cards: cards.to_string(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn it_parses_input() {
        // Given / When
        let hands: Vec<Hand> = INPUT.lines().filter_map(|h| h.parse().ok()).collect();

        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: "32T3K".to_string(),
                    bid: 765,
                },
                Hand {
                    cards: "T55J5".to_string(),
                    bid: 684
                },
                Hand {
                    cards: "KK677".to_string(),
                    bid: 28
                },
                Hand {
                    cards: "KTJJT".to_string(),
                    bid: 220
                },
                Hand {
                    cards: "QQQJA".to_string(),
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn it_solves_pt1() {
        // G
        let input = INPUT.lines().map(|l| l.parse().unwrap()).collect();

        let answer = solve_pt1(input);

        assert_eq!(answer, 6440)
    }

    #[test]
    fn it_solves_pt2() {
        // G
        let input = INPUT.lines().map(|l| l.parse().unwrap()).collect();

        let answer = solve_pt2(input);

        assert_eq!(answer, 5905)
    }
}
