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
        let bag = line.unwrap();
        let (first_compartment, second_compartment) = bag.split_at(bag.len() / 2);
        acc + duplicate_item_score(first_compartment, second_compartment)
    });

    println!("part 1: {score}");
}

fn duplicate_item_score(first_compartment: &str, second_compartment: &str) -> usize {
    first_compartment
        .chars()
        .enumerate()
        .fold(0, |this_score, (i, item)| {
            if !first_compartment[..i].contains(item) && second_compartment.contains(item) {
                this_score + PRIORITIES.find(item).unwrap()
            } else {
                this_score
            }
        })
}

fn pt2() {
    let file = File::open("./data/03.input").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
    let score = lines.chunks(3).fold(0, |acc, bags| acc + badge_score(bags));
    println!("part 2: {score}");
}

fn badge_score(bags: &[String]) -> usize {
    bags[0]
        .chars()
        .enumerate()
        .fold(0, |score, (i, item)| {
            if !bags[0][..i].contains(item) && bags[1].contains(item) && bags[2].contains(item) {
                score + PRIORITIES.find(item).unwrap()
            } else {
                score
            }
        })
}
