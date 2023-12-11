use std::{fs, ops};

fn main() {
    let input = &fs::read_to_string("./data/11.input").unwrap();

    let answer_pt1 = solve(input, 2);
    println!("Part 1: {answer_pt1}");

    let answer_pt2 = solve(input, 1000000);
    println!("Part 2: {answer_pt2}");
}

#[derive(Debug, Clone)]
struct Loc {
    row: i64,
    col: i64,
}

fn solve(input: &str, scale_factor: i64) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (expanded_rows, expanded_cols) = get_expanded(&grid);

    let galaxy_locations = get_galaxy_locations(&grid);
    let galaxy_location_pairs = make_all_pairs(galaxy_locations);

    galaxy_location_pairs
        .into_iter()
        .map(|(mut g1, mut g2)| {
            g1.row += (scale_factor -1) * expanded_rows
                .iter()
                .position(|range| range.contains(&g1.row))
                .unwrap() as i64;

            g1.col += (scale_factor -1) * expanded_cols
                .iter()
                .position(|range| range.contains(&g1.col))
                .unwrap() as i64;

            g2.row += (scale_factor -1) * expanded_rows
                .iter()
                .position(|range| range.contains(&g2.row))
                .unwrap() as i64;

            g2.col += (scale_factor-1) * expanded_cols
                .iter()
                .position(|range| range.contains(&g2.col))
                .unwrap() as i64;

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
                        row: r_idx as i64,
                        col: c_idx as i64,
                    });
                }
            });
            acc
        })
}

fn get_expanded(grid: &[Vec<char>]) -> (Vec<ops::Range<i64>>, Vec<ops::Range<i64>>) {
    let mut expanded_cols: Vec<ops::Range<i64>> = (0..grid[0].len())
        .fold(vec![0], |mut acc, col_idx| {
            if grid.iter().all(|row| row[col_idx] == '.') {
                acc.push(col_idx as i64);
            }
            acc
        })
        .windows(2)
        .map(|n| n[0]..n[1])
        .collect();

    expanded_cols.push(expanded_cols.last().unwrap().end..((grid.len() + 1) as i64));

    let mut expanded_rows: Vec<ops::Range<i64>> = grid
        .iter()
        .enumerate()
        .fold(vec![0], |mut acc, (row_idx, row)| {
            if row.iter().all(|c| c == &'.') {
                acc.push(row_idx as i64);
            }
            acc
        })
        .windows(2)
        .map(|n| n[0]..n[1])
        .collect();

    expanded_rows.push(expanded_rows.last().unwrap().end..((grid.len() + 1) as i64));

    (expanded_rows, expanded_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn it_solves_pt1() {

        let answer = solve(INPUT, 2);

        assert_eq!(answer, 374);
    }

    #[test]
    fn it_solves_pt2_10() {

        let answer = solve(INPUT, 10);

        assert_eq!(answer, 1030);
    }

    #[test]
    fn it_solves_pt2_100() {

        let answer = solve(INPUT, 100);

        assert_eq!(answer, 8410);
    }
}
