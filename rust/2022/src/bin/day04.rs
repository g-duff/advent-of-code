use std::{fs, str};

fn main() {
    pt1();
    pt2();
}

fn pt1() {
    let infile = load("./data/04.input".to_string());

    let total_contained = infile.into_iter().fold(0, |total, ass_pair| {
        total + (ass_pair.fully_contains() as i32)
    });

    println!("Part 1: {total_contained}");
}

fn pt2() {
    let infile = load("./data/04.input".to_string());

    let total_contained = infile.into_iter().fold(0, |total, ass_pair| {
        total + (ass_pair.fully_contains() as i32)
    });

    println!("Part 1: {total_contained}");
}

#[derive(Debug)]
struct SectionAssignmentPair {
    first_assignment: [i32; 2],
    second_assignment: [i32; 2],
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSectionAssignmentPairError;

impl str::FromStr for SectionAssignmentPair {
    type Err = ParseSectionAssignmentPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Vec<i32> = s
            .split(",")
            .map(|l| l.split("-"))
            .flatten()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();

        Ok(Self {
            first_assignment: [bits[0], bits[1]],
            second_assignment: [bits[2], bits[3]],
        })
    }
}

impl SectionAssignmentPair {
    fn fully_contains(self) -> bool {
        (self.first_assignment[0] >= self.second_assignment[0]
            && self.first_assignment[1] <= self.second_assignment[1])
            || (self.second_assignment[0] >= self.first_assignment[0]
                && self.second_assignment[1] <= self.first_assignment[1])
    }
}

fn load(path: String) -> Vec<SectionAssignmentPair> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<SectionAssignmentPair>().ok())
        .collect()
}
