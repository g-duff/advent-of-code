use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = "./data/01.input";

    let file = File::open(input_path).unwrap();
    let pt1_ans = pt1(file);

    println!("Part 1: {}", pt1_ans);
}

fn pt1(file: File) -> i32 {
    let mut tot = 0;
    let mut max = 0;

    BufReader::new(file).lines().for_each(|line| {
        let val = line.unwrap();
        if val == "" {
            if tot > max {
                max = tot;
            }
            tot = 0;
        } else {
            tot += val.parse::<i32>().unwrap();
        }
    });

    max
}
