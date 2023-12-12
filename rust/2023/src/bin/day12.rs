use std::fs;

fn main() {
    let input = fs::read_to_string("./data/12.input").unwrap();
    let answer_pt1 = solve_pt1(&input);
    println!("{}", answer_pt1);
}

fn solve_pt1(input: &str) -> i32 {
    let mut counter = 0;
    for line in input.lines() {
        let (pattern, nums) = line.split_once(' ').unwrap();
        let broken_springs: Vec<i32> = nums.split(',').filter_map(|n| n.parse().ok()).collect();

        let unknown_location: Vec<usize> = pattern
            .match_indices(|c| c == '?')
            .map(|(i, _c)| i)
            .collect();

        let mut str_pattern = pattern.replace(['.', '?'], " ");

        for option in 0..2_usize.pow(unknown_location.len() as u32) {
            unknown_location
                .iter()
                .enumerate()
                .for_each(|(n, l)| match (option >> n) & 1 {
                    0 => str_pattern.replace_range(l..&(l + 1), " "),
                    1 => str_pattern.replace_range(l..&(l + 1), "#"),
                    _ => panic!("bad binary!"),
                });
            let found_broken_springs: Vec<i32> = str_pattern
                .split_whitespace()
                .map(|s| s.len() as i32)
                .collect();
            if found_broken_springs == broken_springs {
                counter += 1;
            }
        }
    }
    counter
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
