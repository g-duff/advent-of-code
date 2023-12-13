use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./data/12.input").unwrap();
    let answer_pt1 = solve_pt1(&input);
    println!("{}", answer_pt1);
    let answer_pt2 = solve_pt2(&input);
    println!("{}", answer_pt2);
}

type Cache = HashMap<(usize, usize, usize), i64>;

fn solve_pt1(input: &str) -> i64 {
    let mut counter = 0;
    for line in input.lines() {
        let mut cache: Cache = HashMap::new();
        let (pattern, nums) = line.split_once(' ').unwrap();
        let broken_springs: Vec<usize> = nums.split(',').filter_map(|n| n.parse().ok()).collect();

        counter += recurse(&mut cache, pattern.to_string(), &broken_springs, 0, 0, 0);
    }
    counter
}

fn solve_pt2(input: &str) -> i64 {
    let mut counter = 0;
    for line in input.lines() {
        let mut cache: Cache = HashMap::new();
        let (pattern, nums) = line.split_once(' ').unwrap();
        let broken_springs: Vec<usize> = nums.split(',').filter_map(|n| n.parse().ok()).collect();

        let super_pattern = [pattern, pattern, pattern, pattern, pattern].join("?");
        let mut super_broken_springs = vec![];
        super_broken_springs.append(&mut broken_springs.clone());
        super_broken_springs.append(&mut broken_springs.clone());
        super_broken_springs.append(&mut broken_springs.clone());
        super_broken_springs.append(&mut broken_springs.clone());
        super_broken_springs.append(&mut broken_springs.clone());

        println!("{}", pattern);
        println!("{:?}", broken_springs);

        counter += recurse(
            &mut cache,
            super_pattern.to_string(),
            &super_broken_springs,
            0,
            0,
            0,
        );
    }
    counter
}

fn recurse(
    cache: &mut Cache,
    in_str: String,
    broken_springs: &Vec<usize>,
    idx_in_line: usize,
    idx_in_broken: usize,
    current_broken: usize,
) -> i64 {
    let key = (idx_in_line, current_broken, idx_in_broken);
    if let Some(x) = cache.get(&key) {
        return *x;
    }

    if idx_in_line == in_str.len() {
        // At the end of the sequence, with all broken sequences complete:
        // or
        // At the end of the sequence, with one broken char left
        if (current_broken == broken_springs.len() && idx_in_broken == 0)
            || (current_broken == broken_springs.len() - 1
                && broken_springs[current_broken] == idx_in_broken)
        {
            1
        // Or incomplete - don't add to count
        } else {
            0
        }
    } else {
        let current_char = in_str.chars().nth(idx_in_line).unwrap();

        let mut res = 0;
        for c in ['.', '#'] {
            if c == current_char || current_char == '?' {
                if c == '.' && idx_in_broken == 0 {
                    res += recurse(
                        cache,
                        in_str.clone(),
                        broken_springs,
                        idx_in_line + 1,
                        0,
                        current_broken,
                    )
                } else if c == '.'
                    && idx_in_broken > 0
                    && current_broken < broken_springs.len()
                    && broken_springs[current_broken] == idx_in_broken
                {
                    res += recurse(
                        cache,
                        in_str.clone(),
                        broken_springs,
                        idx_in_line + 1,
                        0,
                        current_broken + 1,
                    )
                } else if c == '#' {
                    res += recurse(
                        cache,
                        in_str.clone(),
                        broken_springs,
                        idx_in_line + 1,
                        idx_in_broken + 1,
                        current_broken,
                    )
                }
            }
        }
        cache.insert(key, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./../../data/12.example");

    #[test]
    fn it_solves_pt1() {
        let answer = solve_pt1(EXAMPLE);
        assert_eq!(answer, 21);
    }

    #[test]
    fn it_solves_pt2() {
        let answer = solve_pt2(EXAMPLE);
        assert_eq!(answer, 525152);
    }
}
