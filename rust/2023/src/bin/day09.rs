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
    let mut top_row = value_history.clone();
    let mut diffs: Vec<Vec<i32>> = vec![];
    let mut all_zero = false;
    while !all_zero {
        let this_diff: Vec<i32> = top_row.windows(2).map(|n| n[1] - n[0]).collect();
        all_zero = this_diff.iter().all(|d| *d == 0);
        top_row = this_diff.clone();
        diffs.push(this_diff);
    }

    let first_val = value_history.last().unwrap();
    let diff = diffs.iter().map(|d| d.last().unwrap_or(&0)).sum::<i32>();
    first_val + diff
}

fn extrapolate_first_item(value_history: Vec<i32>) -> i32 {
    let mut top_row = value_history.clone();
    let mut diffs: Vec<Vec<i32>> = vec![];
    let mut all_zero = false;
    while !all_zero {
        let this_diff: Vec<i32> = top_row.windows(2).map(|n| n[0] - n[1]).collect();
        all_zero = this_diff.iter().all(|d| *d == 0);
        top_row = this_diff.clone();
        diffs.push(this_diff);
    }

    let first_val = value_history.first().unwrap();
    let diff = diffs.iter().map(|d| d.first().unwrap_or(&0)).sum::<i32>();

    first_val + diff
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
