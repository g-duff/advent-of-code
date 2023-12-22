use std::fs;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("./data/01.input").unwrap();

    let ans_pt1 = solve_pt1(&input);
    println!("Part 1: {}", ans_pt1);

    let ans_pt2 = solve_pt2(&input);
    println!("Part 2: {}", ans_pt2);
}

fn solve_pt1(input: &str) -> i32 {
    input.lines().fold(0, |acc, val| {
        let digits: Vec<i32> = val
            .chars()
            .filter_map(|c| c.to_string().parse().ok())
            .collect();
        let res = 10 * digits.first().unwrap() + digits.last().unwrap();
        acc + res
    })
}

fn solve_pt2(input: &str) -> i32 {
    let answer = input.lines().fold(0, |acc, line| {
        let first_num_val = line;

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
                        || first_num_val[..(first_num_val.len() - i)]
                            .ends_with(&(n + 1).to_string()))
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

    answer
}
