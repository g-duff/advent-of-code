use std::{cmp, fs};

fn main() {
    let all_mirrors = fs::read_to_string("./data/13.input").unwrap();

    let ans = solve_pt1(&all_mirrors);
    println!("{}", ans);
}

fn solve_pt1(input: &str) -> usize {
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

            let do_they_mirror = grid[lesser_n_start..=lesser_n_end]
                .iter()
                .zip(grid[greater_n_start..=greater_n_end].iter().rev())
                .all(|(a, b)| a == b);
            if do_they_mirror {
                count += 100 * (n + 1)
            }
        }

        for n in 0..n_cols - 1 {
            let lesser_n_end = n;
            let greater_n_start = n + 1;
            let slice_length = cmp::min(lesser_n_end, n_cols - n - 2);

            let lesser_n_start = lesser_n_end - slice_length;
            let greater_n_end = greater_n_start + slice_length;

            let do_they_mirror = grid.iter().all(|row| {
                row[lesser_n_start..=lesser_n_end]
                    .iter()
                    .zip(row[greater_n_start..=greater_n_end].iter().rev())
                    .all(|(a, b)| a == b)
            });

            if do_they_mirror {
                count += n + 1
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_solves_pt1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let answer = solve_pt1(input);

        assert_eq!(answer, 405);
    }
}
