use std::str;

fn main() {
    let input = aoc::load_to_vec("./data/18.input");

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

fn solve_pt1(plan_items: &[PlanItem]) -> i64 {
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

fn solve_pt2(plan_items: &[PlanItem]) -> i64 {
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
    let determinant = vertices
        .windows(2)
        .map(|locs| locs[0].row * locs[1].col - locs[0].col * locs[1].row)
        .sum::<i64>();
        

    determinant.abs() / 2
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./../../data/18.example");

    #[test]
    fn it_solves_pt1() {
        let input: Vec<PlanItem> = EXAMPLE.lines().filter_map(|l| l.parse().ok()).collect();
        let ans = solve_pt1(&input);
        assert_eq!(ans, 62);
    }

    #[test]
    fn it_solves_pt2() {
        let input: Vec<PlanItem> = EXAMPLE.lines().filter_map(|l| l.parse().ok()).collect();
        let ans = solve_pt2(&input);
        assert_eq!(ans, 952408144115);
    }
}
