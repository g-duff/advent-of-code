use std::fs;

fn main() {
    let input = fs::read_to_string("./data/14.input").unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let ncols = grid[0].len();
    let nrows = grid.len();

    for c in 0..ncols {
        for r in 0..nrows {
            let mut i = r;
            while i > 0 && i < ncols && grid[i][c] == 'O' && grid[i - 1][c] == '.' {
                grid[i][c] = '.';
                grid[i - 1][c] = 'O';
                i -= 1;
            }
        }
    }

    let ans: usize = grid
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| {
            let rock_count = row.iter().filter(|c| *c == &'O').count();
            return rock_count * (i + 1);
        })
        .sum();

    println!("{}", ans);
}
