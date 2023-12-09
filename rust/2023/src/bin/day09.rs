use std::fs;

fn main() {
    let input = &fs::read_to_string("./data/09.input").unwrap();

    let answer_pt1 = solve_pt1(input);
    println!("Part 1: {}", answer_pt1);

    let answer_pt2 = solve_pt2(input);
    println!("Part 2: {}", answer_pt2);
}

fn solve_pt1(input: &str) -> i32 {
    input
        .lines()
        .map(parse_line_to_numbers)
        .map(extrapolate_last_item)
        .sum()
}

fn solve_pt2(input: &str) -> i32 {
    input
        .lines()
        .map(parse_line_to_numbers)
        .map(extrapolate_first_item)
        .sum()
}

fn parse_line_to_numbers(l: &str) -> Vec<i32> {
    l.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn extrapolate_last_item(value_history: Vec<i32>) -> i32 {
    let mut diffs = vec![value_history];
    while diffs.last().unwrap().iter().any(|d| *d != 0) {
        let next_row = diffs
            .last()
            .unwrap()
            .windows(2)
            // Intent: learn language features. Not necessary for solution
            .filter_map(|w| <&[i32; 2]>::try_from(w).ok())
            .map(|[former, later]| later - former)
            .collect();
        diffs.push(next_row);
    }

    diffs.iter().map(|d| d.last().unwrap_or(&0)).sum::<i32>()
}

fn extrapolate_first_item(value_history: Vec<i32>) -> i32 {
    let mut diffs = vec![value_history];
    while diffs.last().unwrap().iter().any(|d| *d != 0) {
        let next_row = diffs
            .last()
            .unwrap()
            .windows(2)
            .map(|n| n[0] - n[1])
            .collect();
        diffs.push(next_row);
    }

    diffs.iter().map(|d| d.first().unwrap_or(&0)).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn it_solves_pt1() {
        // Given / When
        let answer = solve_pt1(INPUT);

        // Then
        assert_eq!(answer, 114);
    }

    #[test]
    fn it_solves_pt2() {
        // Given / When
        let answer = solve_pt2(INPUT);

        // Then
        assert_eq!(answer, 2);
    }
}
