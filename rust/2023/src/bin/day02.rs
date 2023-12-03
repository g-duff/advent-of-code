use std::{cmp, str};

fn main() {
    let games: Vec<Game> = aoc::load_to_vec("./data/02.input".to_string());

    let pt1_answer: u32 = games
        .iter()
        .filter_map(|g| if g.is_pt1_allowed() { Some(g.id) } else { None })
        .sum();

    let pt2_answer: u32 = games
        .iter()
        .map(|g| g.fewest_possible_cubes().power_of_cubes())
        .sum();

    println!("Part 1: {pt1_answer}");
    println!("Part 2: {pt2_answer}");
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_pt1_allowed(&self) -> bool {
        self.sets
            .iter()
            .map(|s| s.is_pt1_allowed())
            .reduce(|acc, b| acc && b)
            .unwrap()
    }

    fn fewest_possible_cubes(&self) -> Set {
        let mut red = 0u32;
        let mut green = 0u32;
        let mut blue = 0u32;

        self.sets.iter().for_each(|s| {
            red = cmp::max(s.red, red);
            green = cmp::max(s.green, green);
            blue = cmp::max(s.blue, blue);
        });

        Set { red, green, blue }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl str::FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Game, Self::Err> {
        let (game_id, game_sets) = s.split_once(':').unwrap();

        let sets: Vec<Set> = game_sets
            .split(";")
            .map(|set| set.parse::<Set>().unwrap())
            .collect();

        let id = game_id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Ok(Game { id, sets })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn is_pt1_allowed(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power_of_cubes(&self) -> u32 {
        &self.red * &self.green * &self.blue
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSetError;

impl str::FromStr for Set {
    type Err = ParseSetError;

    fn from_str(s: &str) -> Result<Set, ParseSetError> {
        let mut blue = 0u32;
        let mut green = 0u32;
        let mut red = 0u32;

        s.split(",").into_iter().for_each(|color_count| {
            let (count, color) = color_count.trim().split_once(' ').unwrap();
            match color.trim() {
                "red" => red = count.parse::<u32>().unwrap(),
                "green" => green = count.parse::<u32>().unwrap(),
                "blue" => blue = count.parse::<u32>().unwrap(),
                _ => println!("Unknown color!"),
            }
        });

        Ok(Set { red, green, blue })
    }
}

