use std::fs;

const NON_SYMBOL: char = '.';
const GEAR: char = '*';

fn main() {
    let input = fs::read_to_string("./data/03.input").unwrap();
    let pt1_answer = solve_pt1(&input);
    println!("{pt1_answer}");

    let pt2_answer = solve_pt2(&input);
    println!("{pt2_answer}");
}

fn solve_pt1(s: &str) -> i32 {
    let grid = input_str_to_padded_grid(s);

    let mut part_numbers: Vec<i32> = vec![];
    let mut part_number = String::new();
    let mut is_part_number = false;

    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[0].len() - 1) {
            let current_char = &grid[row][col];

            let next_row = row + 1;
            let prev_row = row - 1;

            let next_col = col + 1;
            let prev_col = col - 1;

            let coords = [
                (prev_row, col),
                (prev_row, prev_col),
                (prev_row, next_col),
                (next_row, col),
                (next_row, prev_col),
                (next_row, next_col),
                (row, prev_col),
                (row, next_col),
            ];

            if current_char.is_ascii_digit() {
                part_number.push(*current_char);
                is_part_number =
                    is_part_number || coords.iter().any(|(r, c)| is_not_blank(&grid[*r][*c]));
            }
            if !grid[row][next_col].is_ascii_digit() && !part_number.is_empty() {
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
    c != &NON_SYMBOL && !c.is_ascii_digit()
}

fn solve_pt2(s: &str) -> i32 {
    let grid = input_str_to_padded_grid(s);

    let mut gear_ratios: Vec<i32> = vec![];

    for row in 0..grid.len() {
        for col in 1..(grid[0].len() - 1) {
            if grid[row][col] == GEAR {
                let next_row = row + 1;
                let prev_row = row - 1;

                let next_col = col + 1;
                let prev_col = col - 1;

                let mut part_numbers: Vec<i32> = vec![];
                if grid[row][next_col].is_ascii_digit() {
                    part_numbers.push(search_for_num(&grid, row, next_col));
                }
                if grid[row][prev_col].is_ascii_digit() {
                    part_numbers.push(search_for_num(&grid, row, prev_col));
                }

                if grid[next_row][col].is_ascii_digit() {
                    part_numbers.push(search_for_num(&grid, next_row, col));
                } else {
                    if grid[next_row][next_col].is_ascii_digit() {
                        part_numbers.push(search_for_num(&grid, next_row, next_col));
                    }
                    if grid[next_row][prev_col].is_ascii_digit() {
                        part_numbers.push(search_for_num(&grid, next_row, prev_col));
                    }
                }

                if grid[prev_row][col].is_ascii_digit() {
                    part_numbers.push(search_for_num(&grid, prev_row, col));
                } else {
                    if grid[prev_row][next_col].is_ascii_digit() {
                        part_numbers.push(search_for_num(&grid, prev_row, next_col));
                    }
                    if grid[prev_row][prev_col].is_ascii_digit() {
                        part_numbers.push(search_for_num(&grid, prev_row, prev_col));
                    }
                }

                if part_numbers.len() == 2 {
                    println!("{:?}", part_numbers);
                    gear_ratios.push(part_numbers[0] * part_numbers[1]);
                }
            }
        }
    }
    gear_ratios.iter().sum()
}

fn search_for_num(grid: &[Vec<char>], row: usize, col: usize) -> i32 {
    let mut collected_digits = grid[row][col].to_string();
    let mut i;

    i = 1;
    while grid[row][col + i].is_ascii_digit() {
        collected_digits.push(grid[row][col + i]);
        i += 1;
    }

    i = 1;
    while grid[row][col - i].is_ascii_digit() {
        collected_digits.insert_str(0, &grid[row][col - i].to_string());
        i += 1;
    }
    collected_digits.parse().unwrap()
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

    #[rustfmt::skip]
    const EXAMPLE: &str = include_str!("./../../data/03.example");

    #[test]
    fn it_solves_pt1_example() {
        let result = solve_pt1(EXAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn it_solves_pt2_example() {
        let result = solve_pt2(EXAMPLE);
        assert_eq!(result, 467835);
    }
}
