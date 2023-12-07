use std::{fs, str};

fn main() {
    let input = &fs::read_to_string("./data/07.input").unwrap();

    let pt1_hands: Vec<Hand> = input.lines().filter_map(|l| l.parse().ok()).collect();
    let pt2_hands = pt1_hands.clone();

    let pt1_answer = solve_pt1(pt1_hands);
    println!("{pt1_answer}");

    let pt2_answer = solve_pt2(pt2_hands);
    println!("{pt2_answer}");
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: i32,
}

fn solve_pt1(mut hands: Vec<Hand>) -> i32 {
    hands.sort_unstable_by_key(|h| h.play());
    hands.iter().zip(1..).map(|(h, i)| i * h.bid).sum()
}

fn solve_pt2(mut hands: Vec<Hand>) -> i32 {
    hands.sort_unstable_by_key(|h| h.play_jokers());
    hands.iter().zip(1..).map(|(h, i)| i * h.bid).sum()
}

impl Hand {
    fn play(&self) -> i32 {
        let card_values: Vec<i32> = self
            .cards
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap() as i32,
            })
            .collect();

        let minor_rank = card_values
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, a)| acc + (15_i32.pow(i as u32) * a));

        let mut card_counts = card_values.iter().fold([0; 15], |mut acc, v| {
            acc[*v as usize] += 1;
            acc
        });

        card_counts.sort();
        let major_rank = match &card_counts[11..15] {
            [0, 0, 0, 5] => 6 * 11390625,
            [0, 0, 1, 4] => 5 * 11390625,
            [0, 0, 2, 3] => 4 * 11390625,
            [0, 1, 1, 3] => 3 * 11390625,
            [0, 1, 2, 2] => 2 * 11390625,
            [1, 1, 1, 2] => 11390625,
            _ => 0,
        };

        major_rank + minor_rank
    }

    fn play_jokers(&self) -> i32 {
        let card_values: Vec<i32> = self
            .cards
            .chars()
            .map(|c| match c {
                'J' => 0,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap() as i32,
            })
            .collect();

        let minor_rank = card_values
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, a)| acc + (15_i32.pow(i as u32) * a));

        let mut card_counts = card_values.iter().fold([0; 15], |mut acc, v| {
            acc[*v as usize] += 1;
            acc
        });

        let j = card_counts[0];
        card_counts[0] = 0;

        card_counts.sort();
        card_counts[14] += j;

        let major_rank = match &card_counts[11..15] {
            [0, 0, 0, 5] => 6 * 11390625,
            [0, 0, 1, 4] => 5 * 11390625,
            [0, 0, 2, 3] => 4 * 11390625,
            [0, 1, 1, 3] => 3 * 11390625,
            [0, 1, 2, 2] => 2 * 11390625,
            [1, 1, 1, 2] => 11390625,
            _ => 0,
        };

        major_rank + minor_rank
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl str::FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let (cards, bid) = s.split_once(' ').unwrap();

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

        // Then
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
        // Given
        let input = INPUT.lines().map(|l| l.parse().unwrap()).collect();

        // When
        let answer = solve_pt1(input);

        // Then
        assert_eq!(answer, 6440)
    }

    #[test]
    fn it_solves_pt2() {
        // Given
        let input = INPUT.lines().map(|l| l.parse().unwrap()).collect();

        // When
        let answer = solve_pt2(input);

        // Then
        assert_eq!(answer, 5905)
    }
}
