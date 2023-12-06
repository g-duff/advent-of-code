use std::{fs, iter, str};

fn main() {
    let input = &fs::read_to_string("./data/06.input").unwrap();

    let mini_races = parse_races(input);
    let answer_pt1 = solve_pt1(&mini_races);
    println!("Part 1: {}", answer_pt1);

    let big_race: Race = input.parse().unwrap();
    let answer_pt2 = solve_race(&big_race);
    println!("Part 2: {}", answer_pt2);
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    distance: i64,
    time: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRaceError;

impl str::FromStr for Race {
    type Err = ParseRaceError;

    fn from_str(s: &str) -> Result<Race, ParseRaceError> {
        let (time_line, distance_line) = s.split_once('\n').unwrap();

        let (_time_label, time_numbers) = time_line.split_once(':').unwrap();
        let time = time_numbers.replace(' ', "").parse().unwrap();

        let (_distance_label, distance_numbers) = distance_line.split_once(':').unwrap();
        let distance = distance_numbers.trim().replace(' ', "").parse().unwrap();

        Ok(Race { time, distance })
    }
}

fn solve_pt1(races: &[Race]) -> i64 {
    races.iter().map(solve_race).product()
}

fn solve_race(race: &Race) -> i64 {
    let a = 1.0;
    let b = -race.time as f64;
    let c = race.distance as f64;

    let (max_root, min_root) = solve_quadratic(a, b, c);

    let smallest_hold_time = {
        if min_root.fract() == 0.0 {
            (min_root + 1.0) as i64
        } else {
            min_root.ceil() as i64
        }
    };

    let largest_hold_time = {
        if max_root.fract() == 0.0 {
            (max_root - 1.0) as i64
        } else {
            max_root.floor() as i64
        }
    };

    largest_hold_time - smallest_hold_time + 1
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let max_root = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let min_root = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    (max_root, min_root)
}

fn parse_races(s: &str) -> Vec<Race> {
    let (time_line, distance_line) = s.split_once('\n').unwrap();

    let times = time_line
        .split_whitespace()
        .skip(1)
        .filter_map(|t| t.parse::<i64>().ok());

    let distances = distance_line
        .split_whitespace()
        .skip(1)
        .filter_map(|d| d.parse::<i64>().ok());

    iter::zip(distances, times)
        .map(|(distance, time)| Race { distance, time })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_parses_input() {
        // Given / When
        let output = parse_races(INPUT);

        // Then
        assert_eq!(
            output,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]
        );
    }

    #[test]
    fn it_solves_quadratic() {
        // Given
        let a = 1.0;
        let b = 10.0;
        let c = 5.0;

        // When
        let (root_1, root_2) = solve_quadratic(a, b, c);

        // When
        let y_1 = a * root_1.powf(2.0) + b * root_1 + c;
        let y_2 = a * root_2.powf(2.0) + b * root_2 + c;

        // Then
        assert_eq!(y_1.round(), 0.0);
        assert_eq!(y_2.round(), 0.0);
    }

    #[test]
    fn it_solves_pt_1() {
        // Given
        let races = parse_races(INPUT);

        // When
        let answer = solve_pt1(&races);

        // Then
        assert_eq!(answer, 288)
    }

    #[test]
    fn it_parses_for_pt_2() {
        // Given / When
        let race: Race = INPUT.parse().unwrap();

        // Then
        assert_eq!(
            race,
            Race {
                time: 71530,
                distance: 940200
            }
        );
    }

    #[test]
    fn it_solves_pt_2() {
        // Given
        let race: Race = INPUT.parse().unwrap();

        // When
        let answer = solve_race(&race);

        // Then
        assert_eq!(answer, 71503);
    }
}
