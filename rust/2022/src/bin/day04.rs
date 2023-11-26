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
        let (first_ass, second_ass) = s.split_once(",").unwrap();

        let (first_start, first_end) = first_ass.split_once("-").unwrap();
        let (second_start, second_end) = second_ass.split_once("-").unwrap();

        Ok(Self {
            first_assignment: [
                first_start.parse::<i32>().unwrap(),
                first_end.parse::<i32>().unwrap(),
            ],
            second_assignment: [
                second_start.parse::<i32>().unwrap(),
                second_end.parse::<i32>().unwrap(),
            ],
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
