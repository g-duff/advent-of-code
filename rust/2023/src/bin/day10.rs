use std::fs;
fn main() {
    let input = &fs::read_to_string("./data/10.input").unwrap();

    let answer_pt1 = solve_pt1(input);
    println!("Part 1: {answer_pt1}");
}

fn solve_pt1(input: &str) -> i32 {
    let ground_pipes = input_to_padded_grid(input);

    let row: usize = ground_pipes
        .iter()
        .position(|row| row.contains(&'S'))
        .unwrap();

    let col: usize = ground_pipes[row]
        .iter()
        .position(|col| col == &'S')
        .unwrap();

    let mut previous_position: Location;
    let mut current_position = Location { row, col };

    if "|7F".contains(ground_pipes[current_position.row + 1][current_position.col]) {
        previous_position = current_position.clone();
        current_position.row += 1;
    } else if "|LJ".contains(ground_pipes[current_position.row - 1][current_position.col]) {
        previous_position = current_position.clone();
        current_position.row -= 1;
    } else if "-J7".contains(ground_pipes[current_position.row][current_position.col + 1]) {
        previous_position = current_position.clone();
        current_position.col += 1;
    } else if "-LF".contains(ground_pipes[current_position.row][current_position.col - 1]) {
        previous_position = current_position.clone();
        current_position.col -= 1;
    } else {
        panic!("No next step found!");
    }
    // While current position lookup is not 'S', keep moving and count!
    let mut counter = 1;
    while ground_pipes[current_position.row][current_position.col] != 'S' {
        counter += 1;
        match ground_pipes[current_position.row][current_position.col] {
            '|' => {
                if current_position.row > previous_position.row {
                    previous_position = current_position.clone();
                    current_position.row += 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.row -= 1;
                }
            }
            '-' => {
                if current_position.col > previous_position.col {
                    previous_position = current_position.clone();
                    current_position.col += 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.col -= 1;
                }
            }
            '7' => {
                if current_position.row < previous_position.row {
                    previous_position = current_position.clone();
                    current_position.col -= 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.row += 1;
                }
            }
            'F' => {
                if current_position.row < previous_position.row {
                    previous_position = current_position.clone();
                    current_position.col += 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.row += 1;
                }
            }
            'L' => {
                if current_position.row > previous_position.row {
                    previous_position = current_position.clone();
                    current_position.col += 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.row -= 1;
                }
            }
            'J' => {
                if current_position.row > previous_position.row {
                    previous_position = current_position.clone();
                    current_position.col -= 1;
                } else {
                    previous_position = current_position.clone();
                    current_position.row -= 1;
                }
            }
            _ => panic!("No suitable next move found!"),
        }
    }
    counter / 2
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Location {
    row: usize,
    col: usize,
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
}
