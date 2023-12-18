use std::{fmt, fs};

fn main() {
    let input = fs::read_to_string("./data/14.input").unwrap();

    let ans_pt1 = solve_pt1(&input);
    println!("{}", ans_pt1);

    let ans_pt2 = solve_pt2(&input);
    println!("{}", ans_pt2);
}

fn solve_pt2(input: &str) -> usize {
    let mut grid = Grid {
        data: input.lines().map(|l| l.chars().collect()).collect(),
    };

    let mut states = vec![];
    let mut repeats_start_at = 0;

    while repeats_start_at == 0 {
        grid.cycle();
        let simple_repr = grid.to_string();
        if states.contains(&simple_repr) {
            repeats_start_at = states.iter().position(|s| s == &simple_repr).unwrap();
        }
        states.push(grid.to_string());
    }

    let offset = states.len() - 1;
    let repeating_cycle_length = offset - repeats_start_at;
    let remainder = (1000000000 - repeats_start_at) % repeating_cycle_length;
    let state_of_interest = repeats_start_at + remainder - 1;

    let grid = Grid {
        data: states[state_of_interest]
            .lines()
            .map(|l| l.chars().collect())
            .collect(),
    };
    grid.north_support_beam_load()
}

fn solve_pt1(input: &str) -> usize {
    let mut grid = Grid {
        data: input.lines().map(|l| l.chars().collect()).collect(),
    };

    grid.tilt_north();
    grid.north_support_beam_load()
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        let ncols = self.data[0].len();
        let nrows = self.data.len();
        for c in 0..ncols {
            for r in 0..nrows {
                let mut i = r;
                while i > 0 && self.data[i][c] == 'O' && self.data[i - 1][c] == '.' {
                    self.data[i][c] = '.';
                    self.data[i - 1][c] = 'O';
                    i -= 1;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let ncols = self.data[0].len();
        let nrows = self.data.len();
        for c in 0..ncols {
            for r in 0..nrows {
                let mut i = c;
                while i > 0 && self.data[r][i] == 'O' && self.data[r][i - 1] == '.' {
                    self.data[r][i] = '.';
                    self.data[r][i - 1] = 'O';
                    i -= 1;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let ncols = self.data[0].len();
        let nrows = self.data.len();
        for c in 0..ncols {
            for r in (0..nrows).rev() {
                let mut i = r;
                while i < ncols - 1 && self.data[i][c] == 'O' && self.data[i + 1][c] == '.' {
                    self.data[i][c] = '.';
                    self.data[i + 1][c] = 'O';
                    i += 1;
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let ncols = self.data[0].len();
        let nrows = self.data.len();
        for c in (0..ncols).rev() {
            for r in 0..nrows {
                let mut i = c;
                while i < ncols - 1 && self.data[r][i] == 'O' && self.data[r][i + 1] == '.' {
                    self.data[r][i] = '.';
                    self.data[r][i + 1] = 'O';
                    i += 1;
                }
            }
        }
    }

    fn north_support_beam_load(&self) -> usize {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                let rock_count = row.iter().filter(|c| *c == &'O').count();
                rock_count * (i + 1)
            })
            .sum()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_list: Vec<String> = self
            .data
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect();
        let repr = str_list.join("\n");
        write!(f, "{}", repr)
    }
}
