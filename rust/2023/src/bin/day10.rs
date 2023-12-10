use std::fs;
fn main() {
    let input = &fs::read_to_string("./data/10.input").unwrap();

    let answer_pt1 = solve_pt1(input);
    println!("Part 1: {answer_pt1}");

    let answer_pt2 = solve_pt2(input);
    println!("Part 2: {answer_pt2}");
}

fn solve_pt1(input: &str) -> i32 {
    let ground_pipes = input_to_padded_grid(input);
    let loop_location = get_loop_location(ground_pipes);
    (loop_location.len() as i32) / 2
}

fn solve_pt2(input: &str) -> i32 {
    let ground_pipes = input_to_padded_grid(input);

    let total_rows = ground_pipes.len();
    let mut loop_location = get_loop_location(ground_pipes);

    let loop_len = loop_location.len();
    let first = loop_location[1].pipe;
    let last = loop_location[loop_len - 1].pipe;

    let mut to_replace = '|';
    if first == '|' && last == '|' {
        to_replace = '|';
    } else if first == '-' && last == '-' {
        to_replace = '-';
    } else if (first == '7' && last == 'J') || first == '-' && last == '|' {
        to_replace = 'F';
    }

    loop_location.retain(|loc| loc.pipe != '-');
    loop_location.sort_unstable_by_key(|loc| (loc.row, loc.col));

    let mut locations_by_row: Vec<Vec<Location>> = vec![vec![]; total_rows];

    for loc in loop_location {
        locations_by_row[loc.row].push(loc);
    }

    locations_by_row.retain(|row| !row.is_empty());
    locations_by_row
        .into_iter()
        .map(|row| {
            let mut thick_bars = vec![];
            let mut previous_letter = '|';
            for mut loc in row {
                if loc.pipe == 'S' {
                    loc.pipe = to_replace;
                }
                match loc.pipe {
                    '|' => thick_bars.push(ThickBar {
                        start: loc.col as i32,
                        end: loc.col as i32,
                        switch: true,
                    }),
                    'F' => thick_bars.push(ThickBar {
                        start: loc.col as i32,
                        end: loc.col as i32,
                        switch: true,
                    }),
                    'L' => thick_bars.push(ThickBar {
                        start: loc.col as i32,
                        end: loc.col as i32,
                        switch: true,
                    }),
                    'J' => {
                        thick_bars.last_mut().unwrap().switch = previous_letter == 'F';
                        thick_bars.last_mut().unwrap().end = loc.col as i32;
                    }
                    '7' => {
                        thick_bars.last_mut().unwrap().switch = previous_letter == 'L';
                        thick_bars.last_mut().unwrap().end = loc.col as i32;
                    }
                    _ => panic!("Invalid"),
                }
                previous_letter = loc.pipe;
            }

            let mut area = 0;
            let mut prev_end = thick_bars[0].end;
            let mut inside = false;
            let mut was_inside = false;
            for b in thick_bars {
                was_inside = inside;
                if b.switch {
                    inside = !inside
                }

                if was_inside {
                    area += b.start - prev_end - 1;
                }

                prev_end = b.end
            }
            area
        })
        .sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ThickBar {
    start: i32,
    end: i32,
    switch: bool,
}

fn get_loop_location(ground_pipes: Vec<Vec<char>>) -> Vec<Location> {
    let row: usize = ground_pipes
        .iter()
        .position(|row| row.contains(&'S'))
        .unwrap();

    let col: usize = ground_pipes[row]
        .iter()
        .position(|col| col == &'S')
        .unwrap();

    let mut previous_positions = vec![];
    let mut current_position = Location {
        row,
        col,
        pipe: 'S',
    };

    if "|7F".contains(ground_pipes[current_position.row - 1][current_position.col]) {
        previous_positions.push(current_position.clone());
        current_position.pipe = ground_pipes[current_position.row - 1][current_position.col];
        current_position.row -= 1;
    } else if "-J7".contains(ground_pipes[current_position.row][current_position.col + 1]) {
        previous_positions.push(current_position.clone());
        current_position.pipe = ground_pipes[current_position.row][current_position.col + 1];
        current_position.col += 1;
    } else if "|LJ".contains(ground_pipes[current_position.row + 1][current_position.col]) {
        previous_positions.push(current_position.clone());
        current_position.pipe = ground_pipes[current_position.row + 1][current_position.col];
        current_position.row += 1;
    } else if "-LF".contains(ground_pipes[current_position.row][current_position.col - 1]) {
        previous_positions.push(current_position.clone());
        current_position.pipe = ground_pipes[current_position.row][current_position.col - 1];
        current_position.col -= 1;
    } else {
        panic!("No next step found!");
    }

    // While current position lookup is not 'S', keep moving and count!
    while current_position.pipe != 'S' {
        let previous_position = previous_positions.last().unwrap();
        match current_position.pipe {
            '|' => {
                if current_position.row > previous_position.row {
                    previous_positions.push(current_position.clone());
                    current_position.row += 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.row -= 1;
                }
            }
            '-' => {
                if current_position.col > previous_position.col {
                    previous_positions.push(current_position.clone());
                    current_position.col += 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.col -= 1;
                }
            }
            '7' => {
                if current_position.row < previous_position.row {
                    previous_positions.push(current_position.clone());
                    current_position.col -= 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.row += 1;
                }
            }
            'F' => {
                if current_position.row < previous_position.row {
                    previous_positions.push(current_position.clone());
                    current_position.col += 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.row += 1;
                }
            }
            'L' => {
                if current_position.row > previous_position.row {
                    previous_positions.push(current_position.clone());
                    current_position.col += 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.row -= 1;
                }
            }
            'J' => {
                if current_position.row > previous_position.row {
                    previous_positions.push(current_position.clone());
                    current_position.col -= 1;
                } else {
                    previous_positions.push(current_position.clone());
                    current_position.row -= 1;
                }
            }
            _ => panic!("No suitable next move found!"),
        }
        current_position.pipe = ground_pipes[current_position.row][current_position.col];
    }
    previous_positions
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Location {
    row: usize,
    col: usize,
    pipe: char,
}

fn input_to_padded_grid(input: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|l| {
            let inner: Vec<char> = l.chars().collect();
            let mut result: Vec<char> = vec!['.'];
            result.extend_from_slice(&inner);
            result.push('.');
            result
        })
        .collect();

    let header: Vec<char> = input[0].iter().map(|_c| '.').collect();
    let footer: Vec<char> = header.clone();

    let mut grid = vec![header];
    grid.extend_from_slice(&input);
    grid.push(footer);

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_pt1_simple() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let answer = solve_pt1(input);

        assert_eq!(answer, 4);
    }

    #[test]
    fn it_solves_pt1_harder() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let answer = solve_pt1(input);

        assert_eq!(answer, 8);
    }

    #[test]
    fn it_solves_pt2_simple() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let answer = solve_pt2(input);

        assert_eq!(answer, 1);
    }

    #[test]
    fn it_solves_pt2_harder() {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
        let answer = solve_pt2(input);

        assert_eq!(answer, 4);
    }

    #[test]
    fn it_solves_pt2_even_harder() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let answer = solve_pt2(input);

        assert_eq!(answer, 8);
    }
}
