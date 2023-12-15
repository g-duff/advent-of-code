use std::str;

fn main() {
    let input = include_str!("./../../data/15.input");

    let ans_pt1 = solve_pt1(input);
    println!("Part 1: {}", ans_pt1);

    let ans_pt2 = solve_pt2(input);
    println!("Part 2: {}", ans_pt2);
}

fn solve_pt1(input: &str) -> u32 {
    input.split(',').map(calc_hash).sum()
}

fn solve_pt2(input: &str) -> u32 {
    let mut boxes: Vec<LensBox> = vec![LensBox { lenses: vec! {} }; 256];

    input
        .split(',')
        .filter_map(|lens| lens.parse::<LensStep>().ok())
        .for_each(|lens| {
            let box_idx = calc_hash(&lens.label) as usize;
            boxes[box_idx].apply(lens);
        });

    boxes
        .iter()
        .enumerate()
        .map(|(b_idx, b)| (b_idx as u32 + 1) * b.focussing_power())
        .sum()
}

fn calc_hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LensBox {
    lenses: Vec<LensStep>,
}

impl LensBox {
    fn apply(&mut self, step: LensStep) {
        match step.op {
            Operation::Upsert => {
                let l_idx = self.lenses.iter().position(|ll| ll.label == step.label);
                match l_idx {
                    Some(idx) => self.lenses[idx].focal_length = step.focal_length,
                    None => self.lenses.push(step),
                }
            }
            Operation::Remove => {
                let l_idx = self.lenses.iter().position(|ll| ll.label == step.label);
                if let Some(idx) = l_idx {
                    self.lenses.remove(idx);
                }
            }
        }
    }

    fn focussing_power(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(l_idx, l)| (l_idx as u32 + 1) * l.focal_length)
            .sum::<u32>()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LensStep {
    label: String,
    focal_length: u32,
    op: Operation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Remove,
    Upsert,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseLensStepError;

impl str::FromStr for LensStep {
    type Err = ParseLensStepError;

    fn from_str(s: &str) -> Result<LensStep, ParseLensStepError> {
        let op = match s.contains('-') {
            true => Operation::Remove,
            false => Operation::Upsert,
        };

        let (label, focal_length) = match op {
            Operation::Remove => (s.replace('-', ""), 0),
            Operation::Upsert => {
                let (l, fl) = s.split_once('=').unwrap();
                let fln: u32 = fl.parse().unwrap();
                (l.to_string(), fln)
            }
        };

        Ok(LensStep {
            focal_length,
            op,
            label,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./../../data/15.example");

    #[test]
    fn it_solves_pt1() {
        println!("{}", EXAMPLE);
        let ans = solve_pt1(EXAMPLE);
        assert_eq!(ans, 1320);
    }

    #[test]
    fn it_solves_pt2() {
        let ans = solve_pt2(EXAMPLE);
        assert_eq!(ans, 145);
    }
}
