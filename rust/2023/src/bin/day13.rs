use std::{cmp, fs};

fn main() {
    let all_mirrors = fs::read_to_string("./data/13.input").unwrap();

    let ans_1 = solve(&all_mirrors, 0);
    println!("Part 1: {}", ans_1);

    let ans_1 = solve(&all_mirrors, 1);
    println!("Part 2: {}", ans_1);
}

fn solve(input: &str, n_different: usize) -> usize {
    let mirrors: Vec<&str> = input.split("\n\n").collect();

    let mut count = 0;
    for mirror in mirrors {
        let grid: Vec<Vec<char>> = mirror.lines().map(|l| l.chars().collect()).collect();

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        for n in 0..n_rows - 1 {
            let lesser_n_end = n;
            let greater_n_start = n + 1;
            let slice_length = cmp::min(lesser_n_end, n_rows - n - 2);

            let lesser_n_start = lesser_n_end - slice_length;
            let greater_n_end = greater_n_start + slice_length;

            let do_they_mirror: usize = grid[lesser_n_start..=lesser_n_end]
                .iter()
                .zip(grid[greater_n_start..=greater_n_end].iter().rev())
                .map(|(a, b)| {
                    a.iter()
                        .zip(b.iter())
                        .map(|(c, d)| if c == d { 0 } else { 1 })
                        .sum::<usize>()
                })
                .sum();
            if do_they_mirror == n_different {
                count += 100 * (n + 1)
            }
        }

        for n in 0..n_cols - 1 {
            let lesser_n_end = n;
            let greater_n_start = n + 1;
            let slice_length = cmp::min(lesser_n_end, n_cols - n - 2);

            let lesser_n_start = lesser_n_end - slice_length;
            let greater_n_end = greater_n_start + slice_length;

            let do_they_mirror: usize = grid
                .iter()
                .map(|row| {
                    row[lesser_n_start..=lesser_n_end]
                        .iter()
                        .zip(row[greater_n_start..=greater_n_end].iter().rev())
                        .map(|(c, d)| if c == d { 0 } else { 1 })
                        .sum::<usize>()
                })
                .sum();

            if do_they_mirror == n_different {
                count += n + 1
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./../../data/13.example");

    #[test]
    fn it_solves_pt1() {
        let answer = solve(EXAMPLE, 0);
        assert_eq!(answer, 405);
    }

    #[test]
    fn it_solves_pt2() {
        let answer = solve(EXAMPLE, 1);
        assert_eq!(answer, 400);
    }
}
