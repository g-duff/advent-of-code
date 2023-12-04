use std::str;

fn main() {
    let cards: Vec<Card> = aoc::load_to_vec("./data/04.input".to_string());

    let pt1_answer = solve_pt1(&cards);
    println!("Part 1: {pt1_answer}");

    let pt2_answer = solve_pt2(&cards);
    println!("Part 2: {pt2_answer}");
}

fn solve_pt1(cards: &Vec<Card>) -> i32 {
    cards
        .iter()
        .map(|card| card.count_wins() as u32)
        .map(|wins| match wins {
            0 => 0,
            _ => 2_i32.pow(wins - 1),
        })
        .sum()
}

fn solve_pt2(cards: &Vec<Card>) -> i32 {
    let mut card_repeats: Vec<i32> = vec![1; cards.len()];

    cards
        .iter()
        .map(|card| card.count_wins())
        .enumerate()
        .for_each(|(i, wins)| {
            for w in 1..(wins + 1) {
                card_repeats[i + w] += card_repeats[i];
            }
        });

    card_repeats.iter().sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers_you_have: Vec<i32>,
}

impl Card {
    fn count_wins(&self) -> usize {
        self.numbers_you_have
            .iter()
            .map(|n| self.winning_numbers.contains(n) as usize)
            .sum::<usize>()
    }
}

#[derive(Debug)]
struct ParseCardError;

impl str::FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Card, ParseCardError> {
        let (_card_id, numbers) = s.split_once(":").unwrap();

        let mut split_numbers = numbers.split(" ");

        let winning_numbers = split_numbers
            .by_ref()
            .take_while(|n| *n != "|")
            .filter_map(|n| n.trim().parse().ok())
            .collect();

        let numbers_you_have = split_numbers
            .filter_map(|n| n.trim().parse().ok())
            .collect();

        Ok(Card {
            numbers_you_have,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[rustfmt::skip]
    const PT1_INPUT: &str = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[rustfmt::skip]
    const PT2_INPUT: &str = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parses_input() {
        // Given
        let first_card = PT1_INPUT.lines().next().unwrap();

        // When
        let card: Card = first_card.parse().unwrap();

        // Then
        assert_eq!(
            card,
            Card {
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers_you_have: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn solves_pt1() {
        // Given
        let cards: Vec<Card> = PT1_INPUT.lines().map(|l| l.parse().unwrap()).collect();

        // When
        let answer = solve_pt1(&cards);

        // Then
        assert_eq!(answer, 13);
    }

    #[test]
    fn solves_pt2() {
        // Given
        let cards: Vec<Card> = PT2_INPUT.lines().map(|l| l.parse().unwrap()).collect();

        // When
        let answer = solve_pt2(&cards);

        // Then
        assert_eq!(answer, 30);
    }
}
