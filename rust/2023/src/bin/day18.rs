use std::{fs, str};

fn main() {
    let input = fs::read_to_string("./data/18.input").unwrap();

    let ans_pt1 = solve_pt1(&input);
    println!("Part 1: {}", ans_pt1);
}

struct PlanItem {
    dir: Direction,
    dist: i32,
}

enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Loc {
    row: i32,
    col: i32,
}

fn solve_pt1(input: &str) -> i32 {
    let plan_items: Vec<PlanItem> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let mut row = 0;
    let mut col = 0;
    let mut vertices = vec![Loc { row, col }];
    let outer_points: i32 = plan_items.iter().map(|v| v.dist).sum();
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

fn shoelace(vertices: &[Loc]) -> i32 {
    let first_term = vertices
        .windows(2)
        .map(|locs| locs[0].row * locs[1].col)
        .sum::<i32>();
    let second_term = vertices
        .windows(2)
        .map(|locs| locs[0].col * locs[1].row)
        .sum::<i32>();

    (first_term - second_term).abs() / 2
}

fn picks_theorem(area: &i32, outer_points: &i32) -> i32 {
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

        let dist: i32 = spl[1].parse().unwrap();

        Ok(PlanItem { dir, dist })
    }
}
