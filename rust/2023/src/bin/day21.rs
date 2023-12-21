use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/21.input").unwrap();

    let pt1_ans = solve_pt1(&input, 64);
    println!("Part 1: {}", pt1_ans);

    let pt2_ans = solve_pt2(&input);
    println!("Part 2: {}", pt2_ans);
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

        let Some(rn) = r.checked_add(1) else { continue };
        if rn < n_rows && !seen.contains(&[rn, c, n + 1]) && grid[rn][c] == '.' {
            seen.insert([rn, c, n + 1]);
            q.push_back([rn, c, n + 1]);
        }

        let Some(rn) = r.checked_sub(1) else { continue };
        if !seen.contains(&[rn, c, n + 1]) && grid[rn][c] == '.' {
            seen.insert([rn, c, n + 1]);
            q.push_back([rn, c, n + 1]);
        }

        let Some(cn) = c.checked_add(1) else { continue };
        if cn < n_cols && !seen.contains(&[r, cn, n + 1]) && grid[r][cn] == '.' {
            seen.insert([r, cn, n + 1]);
            q.push_back([r, cn, n + 1]);
        }

        let Some(cn) = c.checked_sub(1) else { continue };
        if !seen.contains(&[r, cn, n + 1]) && grid[r][cn] == '.' {
            seen.insert([r, cn, n + 1]);
            q.push_back([r, cn, n + 1]);
        }
    }
    plots_reached
}

fn solve_pt2(input: &str) -> i128 {
    let mut data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start_row = data.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = data[start_row].iter().position(|c| c == &'S').unwrap();

    data[start_row][start_col] = '.';

    let n_rows = data.len() as i32;
    let n_cols = data[0].len() as i32;

    let grid = InfiniteGrid {
        data,
        n_rows,
        n_cols,
    };

    let mut seen: HashSet<[i32; 3]> = HashSet::new();
    let mut q: VecDeque<[i32; 3]> = VecDeque::new();

    let x: [i128; 4] = [65, 65 + 131, 65 + 2 * 131, 65 + 202300 * 131];
    let mut y: [i128; 3] = [0; 3];

    q.push_back([start_row as i32, start_col as i32, 0]);

    while let Some([r, c, n]) = q.pop_front() {
        if n as i128 == x[0] && !seen.contains(&[r, c, n + 1]) && grid.get(r, c) == '.' {
            y[0] += 1
        }

        if n as i128 == x[1] && !seen.contains(&[r, c, n + 1]) && grid.get(r, c) == '.' {
            y[1] += 1;
        }

        if n as i128 == x[2] && !seen.contains(&[r, c, n + 1]) && grid.get(r, c) == '.' {
            y[2] += 1;
            seen.insert([r, c, n + 1]);
            continue;
        }

        for [dr, dc] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let rn = r + dr;
            let cn = c + dc;
            if !seen.contains(&[rn, cn, n + 1]) && grid.get(rn, cn) == '.' {
                seen.insert([rn, cn, n + 1]);
                q.push_back([rn, cn, n + 1]);
            }
        }
    }

    // Lagrange Polynomial
    // 128 because of problems with integer overflow
    let ans: i128 = y[0] * (x[3] - x[1]) * (x[3] - x[2]) / ((x[0] - x[1]) * (x[0] - x[2]))
        + y[1] * (x[3] - x[0]) * (x[3] - x[2]) / ((x[1] - x[0]) * (x[1] - x[2]))
        + y[2] * (x[3] - x[0]) * (x[3] - x[1]) / ((x[2] - x[0]) * (x[2] - x[1]));

    ans
}

struct InfiniteGrid {
    data: Vec<Vec<char>>,
    n_rows: i32,
    n_cols: i32,
}

impl InfiniteGrid {
    fn get(&self, r: i32, c: i32) -> char {
        let rr = r.rem_euclid(self.n_rows) as usize;
        let cc = c.rem_euclid(self.n_cols) as usize;
        self.data[rr][cc]
    }
}
