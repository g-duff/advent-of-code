use std::cmp;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/17.input").unwrap();

    let ans_pt1 = solve_pt1(&input).unwrap();
    println!("{}", ans_pt1);

    let ans_pt2 = solve_pt2(&input).unwrap();
    println!("{}", ans_pt2);
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    r: i32,
    c: i32,
    dr: i32,
    dc: i32,
    n: usize,
}

#[derive(Eq, PartialEq)]
struct State {
    position: Position,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Flip ordering on costs
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_pt1(input: &str) -> Option<usize> {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_string().parse::<usize>().ok())
                .collect()
        })
        .collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(State {
        cost: 0,
        position: Position {
            r: 0,
            c: 0,
            dr: 0,
            dc: 0,
            n: 0,
        },
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position.r == (nrows - 1) as i32 && position.c == (ncols - 1) as i32 {
            return Some(cost);
        };

        if seen.contains(&position) {
            continue;
        }

        seen.insert(position.clone());

        if position.n < 3 && [position.dr, position.dc] != [0, 0] {
            let r = position.r + position.dr;
            let c = position.c + position.dc;

            if r >= 0 && r < nrows as i32 && c >= 0 && c < ncols as i32 {
                heap.push(State {
                    cost: cost + grid[r as usize][c as usize],
                    position: Position {
                        r,
                        c,
                        dr: position.dr,
                        dc: position.dc,
                        n: position.n + 1,
                    },
                });
            }
        }

        for [dr, dc] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            if [dr, dc] != [position.dr, position.dc] && [dr, dc] != [-position.dr, -position.dc] {
                let r = position.r + dr;
                let c = position.c + dc;

                if r >= 0 && r < nrows as i32 && c >= 0 && c < ncols as i32 {
                    heap.push(State {
                        cost: cost + grid[r as usize][c as usize],
                        position: Position { r, c, dr, dc, n: 1 },
                    });
                }
            }
        }
    }
    None
}

fn solve_pt2(input: &str) -> Option<usize> {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_string().parse::<usize>().ok())
                .collect()
        })
        .collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(State {
        cost: 0,
        position: Position {
            r: 0,
            c: 0,
            dr: 0,
            dc: 0,
            n: 0,
        },
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position.r == (nrows - 1) as i32 && position.c == (ncols - 1) as i32 && position.n >= 4 {
            return Some(cost);
        };

        if seen.contains(&position) {
            continue;
        }

        seen.insert(position.clone());

        if position.n < 10 && [position.dr, position.dc] != [0, 0] {
            let r = position.r + position.dr;
            let c = position.c + position.dc;

            if r >= 0 && r < nrows as i32 && c >= 0 && c < ncols as i32 {
                heap.push(State {
                    cost: cost + grid[r as usize][c as usize],
                    position: Position {
                        r,
                        c,
                        dr: position.dr,
                        dc: position.dc,
                        n: position.n + 1,
                    },
                });
            }
        }

        if position.n >= 4 || [position.dr, position.dc] == [0, 0] {
            for [dr, dc] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
                if [dr, dc] != [position.dr, position.dc]
                    && [dr, dc] != [-position.dr, -position.dc]
                {
                    let r = position.r + dr;
                    let c = position.c + dc;

                    if r >= 0 && r < nrows as i32 && c >= 0 && c < ncols as i32 {
                        heap.push(State {
                            cost: cost + grid[r as usize][c as usize],
                            position: Position { r, c, dr, dc, n: 1 },
                        });
                    }
                }
            }
        }
    }
    None
}
