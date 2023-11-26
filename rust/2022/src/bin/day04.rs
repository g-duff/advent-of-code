use std::{fs, str};

fn main() {
    let infile = load("./data/04.input".to_string());

    infile
        .into_iter()
        .for_each(|line| println!("{:?}, {:?}", line.first_assignment, line.second_assignment));
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

fn load(path: String) -> Vec<SectionAssignmentPair> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<SectionAssignmentPair>().ok())
        .collect()
}
