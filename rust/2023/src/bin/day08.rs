use std::collections::HashMap;
use std::fs;

fn main() {
    let input = &fs::read_to_string("./data/08.input").unwrap();

    let (directions, map) = parse_input(input);

    // let answer_pt1 = solve_pt1(directions, map);
    // println!("Part 1: {answer_pt1}");

    let answer_pt2 = solve_pt2(directions, map);
    println!("Part 2: {answer_pt2}");
}

fn solve_pt2(directions: String, map: HashMap<String, Node>) -> i64 {
    let current_positions: Vec<String> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();

    let counts: Vec<i64> = current_positions
        .iter()
        .map(|mut current_position| {
            let mut counter = 0;

            while !current_position.ends_with('Z') {
                for d in directions.chars() {
                    let current_node = map.get(current_position).unwrap();
                    current_position = match d {
                        'L' => &current_node.left,
                        'R' => &current_node.right,
                        _ => panic!("Invalid input"),
                    };
                    counter += 1;
                }
            }
            counter
        })
        .collect();

    let mut prime_factors: Vec<i64> = counts
        .iter()
        .flat_map(|c| {
            let mut upto = *c;
            if *c < 4 {
                upto += 1
            }
            (1..upto).filter(move |n| c % n == 0)
        })
        .collect();

    prime_factors.sort();
    prime_factors.dedup();
    prime_factors.iter().product()
}

fn solve_pt1(directions: String, map: HashMap<String, Node>) -> i32 {
    let end = "ZZZ";
    let mut current_position = "AAA";
    let mut counter = 0;

    while current_position != end {
        for d in directions.chars() {
            let current_node = map.get(current_position).unwrap();
            current_position = match d {
                'L' => &current_node.left,
                'R' => &current_node.right,
                _ => panic!("Invalid input"),
            };
            counter += 1;
        }
    }
    counter
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (String, HashMap<String, Node>) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().to_string();

    let map = lines
        .skip(1)
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();

            let key = parts[0];
            let left = parts[2][1..4].to_string();
            let right = parts[3][0..3].to_string();
            (key, Node { left, right })
        })
        .fold(HashMap::new(), |mut map, (key, node)| {
            map.insert(key.to_string(), node);
            map
        });
    (directions, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn it_parses_input() {
        // Given / When
        let (directions, map) = parse_input(INPUT);

        // Then
        assert_eq!(directions, "LLR");
        assert_eq!(
            map,
            HashMap::from([
                (
                    "AAA".to_string(),
                    Node {
                        left: "BBB".to_string(),
                        right: "BBB".to_string()
                    }
                ),
                (
                    "BBB".to_string(),
                    Node {
                        left: "AAA".to_string(),
                        right: "ZZZ".to_string()
                    }
                ),
                (
                    "ZZZ".to_string(),
                    Node {
                        left: "ZZZ".to_string(),
                        right: "ZZZ".to_string()
                    }
                ),
            ])
        );
    }

    #[test]
    fn it_solves_pt1() {
        // Given
        let (directions, map) = parse_input(INPUT);

        let answer = solve_pt1(directions, map);

        assert_eq!(answer, 6);
    }

    #[test]
    fn it_solves_pt2() {
        // Given
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let (directions, map) = parse_input(input);

        let answer = solve_pt2(directions, map);

        assert_eq!(answer, 6);
    }
}
