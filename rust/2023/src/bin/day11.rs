use std::{fs, ops, str};

fn main() {
    let input: Universe = fs::read_to_string("./data/11.input")
        .unwrap()
        .parse()
        .unwrap();

    let answer_pt1 = solve(&input, 2);
    println!("Part 1: {answer_pt1}");

    let answer_pt2 = solve(&input, 1000000);
    println!("Part 2: {answer_pt2}");
}

fn solve(input: &Universe, expansion_factor: i64) -> i64 {
    let row_expansion = input.get_row_expansion(expansion_factor);
    let col_expansion = input.get_col_expansion(expansion_factor);

    let galaxy_locations = input.get_galaxy_locations(row_expansion, col_expansion);
    let galaxy_location_pairs = make_all_pairs(galaxy_locations);

    galaxy_location_pairs
        .into_iter()
        .map(|(g1, g2)| {
            let row_distance = (g1.row - g2.row).abs();
            let col_distance = (g1.col - g2.col).abs();

            row_distance + col_distance
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Loc {
    row: i64,
    col: i64,
}

struct Expansion {
    ranges: Vec<ops::Range<i64>>,
    factor: i64,
}

impl Expansion {
    fn apply(&self, x: i64) -> i64 {
        let expanded_rows_count = self
            .ranges
            .iter()
            .position(|range| range.contains(&x))
            .unwrap() as i64;
        x + (self.factor - 1) * expanded_rows_count
    }
}

struct Universe {
    grid: Vec<Vec<char>>,
}

impl Universe {
    fn get_col_expansion(&self, expansion_factor: i64) -> Expansion {
        let mut blank_cols: Vec<i64> = (0..self.grid[0].len())
            .filter(|col_idx| self.grid.iter().all(|row| row[*col_idx] == '.'))
            .map(|col_idx| col_idx as i64)
            .collect();

        blank_cols.insert(0, 0);
        blank_cols.push((self.grid.len() + 1) as i64);

        Expansion {
            ranges: blank_cols.windows(2).map(|n| n[0]..n[1]).collect(),
            factor: expansion_factor,
        }
    }

    fn get_row_expansion(&self, expansion_factor: i64) -> Expansion {
        let mut blank_rows: Vec<i64> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|c| c == &'.'))
            .map(|(row_idx, _)| row_idx as i64)
            .collect();

        blank_rows.insert(0, 0);
        blank_rows.push((self.grid.len() + 1) as i64);

        Expansion {
            ranges: blank_rows.windows(2).map(|n| n[0]..n[1]).collect(),
            factor: expansion_factor,
        }
    }

    fn get_galaxy_locations(&self, row_expansion: Expansion, col_expansion: Expansion) -> Vec<Loc> {
        let mut galaxy_locations = vec![];
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                if c == &'#' {
                    galaxy_locations.push(Loc {
                        row: row_expansion.apply(row_idx as i64),
                        col: col_expansion.apply(col_idx as i64),
                    });
                }
            }
        }
        galaxy_locations
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ParseUniverseError;

impl str::FromStr for Universe {
    type Err = ParseUniverseError;

    fn from_str(input: &str) -> Result<Universe, ParseUniverseError> {
        Ok(Universe {
            grid: input.lines().map(|l| l.chars().collect()).collect(),
        })
    }
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
        let input = INPUT.parse().unwrap();

        let answer = solve(&input, 2);

        assert_eq!(answer, 374);
    }

    #[test]
    fn it_solves_pt2_10() {
        let input = INPUT.parse().unwrap();

        let answer = solve(&input, 10);

        assert_eq!(answer, 1030);
    }

    #[test]
    fn it_solves_pt2_100() {
        let input = INPUT.parse().unwrap();

        let answer = solve(&input, 100);

        assert_eq!(answer, 8410);
    }
}
