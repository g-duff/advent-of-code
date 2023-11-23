use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./data/01.input").unwrap();

    // let pt1_ans = pt1(file);
    // println!("Part 1: {}", pt1_ans);

    let pt2_ans = pt2(file);
    println!("Part 2: {}", pt2_ans);
}

#[allow(dead_code)]
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

fn pt2(file: File) -> i32 {
    let mut tot = 0;
    let mut max = vec![0, 0, 0];

    BufReader::new(file).lines().for_each(|line| {
        let val = line.unwrap();
        if val == "" {
            for i in 0..max.len() {
                if tot > max[i] {
                    max.pop();
                    max.insert(i, tot);
                    break;
                }
            }
            tot = 0;
        } else {
            tot += val.parse::<i32>().unwrap();
        }
    });

    max.into_iter().sum()
}
