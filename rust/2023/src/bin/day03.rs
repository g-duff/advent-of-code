use std::fs::File;
use std::io::{BufRead, BufReader};

const PRIORITIES: &str = "^abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    pt1();
    pt2();
}

fn pt1() {
    let file = File::open("./data/03.input").unwrap();

    let score = BufReader::new(file).lines().fold(0, |acc, line| {
        let bag_contents = line.unwrap();
        let len = bag_contents.len() / 2;
        let (first_compartment, second_compartment) = bag_contents.split_at(len);

        let this_score = first_compartment.chars().enumerate().fold(0, |this_score, (i, item)| {
            if !first_compartment[..i].contains(item) && second_compartment.contains(item) {
                this_score + PRIORITIES.find(item).unwrap()
            } else {
                this_score
            }
        });
        acc + this_score
    });

    println!("part 1: {}", score);
}

fn pt2() {
    let file = File::open("./data/03.input").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let score = lines.chunks(3).fold(0, |acc, bags| {
        let this_score = bags[0]
            .chars()
            .enumerate()
            .fold(0, |this_score, (i, item)| {
                if !bags[0][..i].contains(item) && bags[1].contains(item) && bags[2].contains(item)
                {
                    this_score + PRIORITIES.find(item).unwrap()
                } else {
                    this_score
                }
            });
        acc + this_score
    });
    println!("part 2: {}", score);
}
