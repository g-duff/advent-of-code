use std::{fs, str};

fn main() {
    let input = fs::read_to_string("./data/18.input").unwrap();

    let ans_pt1 = solve_pt1(&input);
    println!("Part 1: {}", ans_pt1);

    let ans_pt2 = solve_pt2(&input);
    println!("Part 2: {}", ans_pt2);
}

#[derive(Debug)]
struct PlanItem {
    dir: Direction,
    dist: i64,
    hex_dist: i64,
    hex_dir: Direction,
}

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Loc {
    row: i64,
    col: i64,
}

fn solve_pt1(input: &str) -> i64 {
    let plan_items: Vec<PlanItem> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let mut row = 0;
    let mut col = 0;
    let mut vertices = vec![Loc { row, col }];
    let outer_points: i64 = plan_items.iter().map(|v| v.dist).sum();
    for p in plan_items {
        match p.dir {
            Direction::U => row -= p.dist,
            Direction::D => row += p.dist,
            Direction::L => col -= p.dist,
            Direction::R => col += p.dist,
        }
        vertices.push(Loc { row, col })
    }

    let area = shoelace(&vertices);
    let enclosed_points = picks_theorem(&area, &outer_points);

    enclosed_points + outer_points
}

fn solve_pt2(input: &str) -> i64 {
    let plan_items: Vec<PlanItem> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let mut row = 0;
    let mut col = 0;
    let mut vertices = vec![Loc { row, col }];
    let outer_points: i64 = plan_items.iter().map(|v| v.hex_dist).sum();
    for p in plan_items {
        match p.hex_dir {
            Direction::U => row -= p.hex_dist,
            Direction::D => row += p.hex_dist,
            Direction::L => col -= p.hex_dist,
            Direction::R => col += p.hex_dist,
        }
        vertices.push(Loc { row, col })
    }

    let area = shoelace(&vertices);
    let enclosed_points = picks_theorem(&area, &outer_points);

    enclosed_points + outer_points
}

fn shoelace(vertices: &[Loc]) -> i64 {
    let first_term = vertices
        .windows(2)
        .map(|locs| locs[0].row * locs[1].col)
        .sum::<i64>();
    let second_term = vertices
        .windows(2)
        .map(|locs| locs[0].col * locs[1].row)
        .sum::<i64>();

    (first_term - second_term).abs() / 2
}

fn picks_theorem(area: &i64, outer_points: &i64) -> i64 {
    area + 1 - outer_points / 2
}

struct ParsePlanItemError;

impl str::FromStr for PlanItem {
    type Err = ParsePlanItemError;

    fn from_str(s: &str) -> Result<PlanItem, ParsePlanItemError> {
        let spl: Vec<&str> = s.split_whitespace().collect();

        let dir = match spl[0] {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => unreachable!(),
        };

        let dist: i64 = spl[1].parse().unwrap();

        let hex_digit = spl[2].trim_start_matches("(#").trim_end_matches(')');

        let hex_dist = i64::from_str_radix(&hex_digit[0..hex_digit.len() - 1], 16).unwrap();

        let hex_dir = match &hex_digit[hex_digit.len() - 1..hex_digit.len()] {
            "0" => Direction::R,
            "1" => Direction::D,
            "2" => Direction::L,
            "3" => Direction::U,
            _ => unreachable!(),
        };

        Ok(PlanItem {
            dir,
            dist,
            hex_dir,
            hex_dist,
        })
    }
}
