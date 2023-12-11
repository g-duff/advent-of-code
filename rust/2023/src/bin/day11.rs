use std::fs;
fn main() {
    let input = &fs::read_to_string("./data/11.input").unwrap();

    let answer_pt1 = solve_pt1(input);
    println!("Part 1: {answer_pt1}");
}

#[derive(Debug, Clone)]
struct Loc {
    row: i32,
    col: i32,
}

fn solve_pt1(input: &str) -> i32 {
    let grid = input_to_grid(input);
    let galaxy_locations = get_galaxy_locations(&grid);
    let galaxy_location_pairs = make_all_pairs(galaxy_locations);

    galaxy_location_pairs
        .iter()
        .map(|(g1, g2)| {
            let row_distance = (g1.row - g2.row).abs();
            let col_distance = (g1.col - g2.col).abs();

            row_distance + col_distance
        })
        .sum()
}

fn make_all_pairs<T>(mut singles: Vec<T>) -> Vec<(T, T)>
where
    T: Clone,
{
    let mut pairs: Vec<(T, T)> = vec![];
    while let Some(g1) = singles.pop() {
        for g2 in singles.iter() {
            pairs.push((g1.clone(), g2.clone()))
        }
    }
    pairs
}

fn get_galaxy_locations(image: &[Vec<char>]) -> Vec<Loc> {
    image
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (r_idx, row)| {
            row.iter().enumerate().for_each(|(c_idx, char)| {
                if char == &'#' {
                    acc.push(Loc {
                        row: r_idx as i32,
                        col: c_idx as i32,
                    });
                }
            });
            acc
        })
}

fn input_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let n_cols = grid[0].len();

    for col_idx in (0..n_cols).rev() {
        if grid.iter().all(|row| row[col_idx] == '.') {
            grid.iter_mut().for_each(|row| row.insert(col_idx, '.'));
        }
    }

    grid = grid.into_iter().fold(vec![], |mut acc, row| {
        if row.iter().all(|c| c == &'.') {
            acc.push(row.clone());
        }
        acc.push(row);
        acc
    });

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_pt1_simple() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let answer = solve_pt1(input);

        assert_eq!(answer, 374);
    }
}
