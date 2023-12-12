use std::fs;

fn main() {
    let input = fs::read_to_string("./data/12.input").unwrap();
    let answer_pt1 = solve_pt1(&input);
    println!("{}", answer_pt1);
}

fn solve_pt1(input: &str) -> i64 {
    let mut counter = 0;
    for line in input.lines() {
        let (pattern, nums) = line.split_once(' ').unwrap();
        let broken_springs: Vec<i32> = nums.split(',').filter_map(|n| n.parse().ok()).collect();

        counter += recurse(pattern.replace('.', " "), &broken_springs, 0);
    }
    counter
}

fn recurse(in_str: String, broken_springs: &Vec<i32>, idx: usize) -> i64 {
    if idx == in_str.len() {
        is_valid(&in_str, broken_springs) as i64
    } else if in_str.chars().nth(idx).unwrap() == '?' {
        recurse(in_str.replacen('?', "#", 1), broken_springs, idx + 1)
            + recurse(in_str.replacen('?', " ", 1), broken_springs, idx + 1)
    } else {
        recurse(in_str, broken_springs, idx + 1)
    }
}

fn is_valid(str_pattern: &str, broken_springs: &Vec<i32>) -> bool {
    let found_broken_springs: Vec<i32> = str_pattern
        .split_whitespace()
        .map(|s| s.len() as i32)
        .collect();
    &found_broken_springs == broken_springs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_solves_pt1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let answer = solve_pt1(input);

        assert_eq!(answer, 21);
    }
}
