use std::fs::File;
use std::io::{BufRead, BufReader};

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    pt1();
    pt2();
}

fn pt1() {
    let file = File::open("./data/01.input").unwrap();

    let answer = BufReader::new(file).lines().fold(0, |acc, line| {
        let val = line.unwrap();
        let digits: Vec<char> = val
            .chars()
            .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
            .collect();

        let res = String::from_iter([digits.first().unwrap(), digits.last().unwrap()])
            .parse::<i32>()
            .unwrap();

        acc + res
    });

    println!("Part 1: {answer}");
}

fn pt2() {
    let file = File::open("./data/01.input").unwrap();

    let answer = BufReader::new(file).lines().fold(0, |acc, line| {
        let first_num_val = line.unwrap();

        let mut first_digit = 0.to_string();
        let mut last_digit = 0.to_string();

        for i in 0..first_num_val.len() {
            for (n, num) in NUMS.iter().enumerate() {

                if first_digit == 0.to_string()
                    && (first_num_val[i..].starts_with(num)
                        || first_num_val[i..].starts_with(&(n + 1).to_string()))
                {
                    first_digit = (n + 1).to_string();
                }

                if last_digit == 0.to_string()
                    && (first_num_val[..(first_num_val.len() - i)].ends_with(num)
                        || first_num_val[..(first_num_val.len() - i)].ends_with(&(n + 1).to_string()))
                {
                    last_digit = (n + 1).to_string();
                }

            }
        }

        let res = String::from_iter([first_digit, last_digit])
            .parse::<i32>()
            .unwrap();

        acc + res
    });

    println!("Part 2: {answer}");
}
