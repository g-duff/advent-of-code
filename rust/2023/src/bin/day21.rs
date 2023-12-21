use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/21.input").unwrap();

    let pt1_ans = solve_pt1(&input, 64);
    println!("Part 1: {}", pt1_ans);
}

fn solve_pt1(input: &str, steps: usize) -> i32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let start_row = grid.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = grid[start_row].iter().position(|c| c == &'S').unwrap();

    grid[start_row][start_col] = '.';

    let mut seen: HashSet<[usize; 3]> = HashSet::new();
    let mut q: VecDeque<[usize; 3]> = VecDeque::new();

    q.push_back([start_row, start_col, 0]);

    let mut plots_reached = 0;

    while let Some([r, c, n]) = q.pop_front() {
        if n == steps && !seen.contains(&[r, c, n + 1]) && grid[r][c] == '.' {
            plots_reached += 1;
            seen.insert([r, c, n + 1]);
            continue;
        }

        if let Some(rn) = r.checked_add(1) {
            if rn < n_rows && !seen.contains(&[rn, c, n + 1]) && grid[rn][c] == '.' {
                seen.insert([rn, c, n + 1]);
                q.push_back([rn, c, n + 1]);
            }
        }

        if let Some(rn) = r.checked_sub(1) {
            if !seen.contains(&[rn, c, n + 1]) && grid[rn][c] == '.' {
                seen.insert([rn, c, n + 1]);
                q.push_back([rn, c, n + 1]);
            }
        }

        if let Some(cn) = c.checked_add(1) {
            if cn < n_cols && !seen.contains(&[r, cn, n + 1]) && grid[r][cn] == '.' {
                seen.insert([r, cn, n + 1]);
                q.push_back([r, cn, n + 1]);
            }
        }

        if let Some(cn) = c.checked_sub(1) {
            if !seen.contains(&[r, cn, n + 1]) && grid[r][cn] == '.' {
                seen.insert([r, cn, n + 1]);
                q.push_back([r, cn, n + 1]);
            }
        }
    }
    plots_reached
}
