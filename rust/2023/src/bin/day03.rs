use std::fs;

const NON_SYMBOL: char = '.';

#[rustfmt::skip]
const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn main() {
    let input = fs::read_to_string("./data/03.input").unwrap();
    let pt1_answer = solve_pt1(&input);
    println!("{pt1_answer}");
}

fn solve_pt1(s: &str) -> i32 {
    let grid = input_str_to_padded_grid(s);

    let rows_count = grid.len();
    let cols_count = grid[0].len();

    let mut part_numbers: Vec<i32> = vec![];
    let mut part_number = String::new();
    let mut is_part_number = false;

    for row_index in 1..(rows_count - 1) {
        for col_index in 1..(cols_count - 1) {
            let current_char = &grid[row_index][col_index];

            let top = &grid[row_index - 1][col_index];
            let top_left = &grid[row_index - 1][col_index - 1];
            let top_right = &grid[row_index - 1][col_index + 1];

            let bottom = &grid[row_index + 1][col_index];
            let bottom_left = &grid[row_index + 1][col_index - 1];
            let bottom_right = &grid[row_index + 1][col_index + 1];

            let left = &grid[row_index][col_index - 1];
            let right = &grid[row_index][col_index + 1];

            if current_char.is_digit(10) {
                part_number.push(*current_char);
                if is_not_blank(top)
                    || is_not_blank(top_left)
                    || is_not_blank(top_right)
                    || is_not_blank(bottom)
                    || is_not_blank(bottom_left)
                    || is_not_blank(bottom_right)
                    || is_not_blank(left)
                    || is_not_blank(right)
                {
                    is_part_number = true;
                }
            }
            if !right.is_digit(10) && part_number != "" {
                println!("{part_number}, {is_part_number}");
                if is_part_number {
                    part_numbers.push(part_number.parse().unwrap());
                }
                part_number = "".to_string();
                is_part_number = false;
            }
        }
    }

    part_numbers.iter().sum()
}

fn is_not_blank(c: &char) -> bool {
    c != &NON_SYMBOL && !c.is_digit(10)
}

fn input_str_to_padded_grid(input: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let inner: Vec<char> = l.chars().collect();
            let mut result: Vec<char> = vec![NON_SYMBOL];
            result.extend_from_slice(&inner);
            result.push(NON_SYMBOL);
            result
        })
        .collect();

    let header: Vec<char> = input[0].iter().map(|_c| '.').collect();
    let footer: Vec<char> = header.clone();

    let mut grid = vec![header];
    grid.extend_from_slice(&input);
    grid.push(footer);

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_pt1_example() {
        // Given When
        let result = solve_pt1(INPUT);

        // Then
        assert_eq!(result, 4361);
    }

    #[test]
    fn it_solves_pt1_input() {
        // Given
        let input = fs::read_to_string("./data/03.input").unwrap();

        // When
        let result = solve_pt1(&input);

        // Then
        assert_eq!(result, 521515);
    }
}