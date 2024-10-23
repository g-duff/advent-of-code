use std::fs;

fn main() {
    let infile = fs::read_to_string("./data/06.input").unwrap();

    let ans_pt1 = solve_pt1(infile.chars().collect());
    println!("Part 1: {}", ans_pt1);
}

fn solve_pt1(datastream_buffer: Vec<char>) -> usize {
    let mut ans = 0;
    for i in 0..datastream_buffer.len() {
        let mut counts = Vec::with_capacity(4);
        for j in i..(i + 4) {
            let count = datastream_buffer[i..(i + 4)]
                .iter()
                .filter(|c| *c == &datastream_buffer[j])
                .count();
            counts.push(count);
        }
        if counts == vec![1, 1, 1, 1] {
            ans = i + 4;
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_solves_pt1_ex1() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let ans = solve_pt1(input.chars().collect());
        assert_eq!(ans, 5);
    }

    #[test]
    fn it_solves_pt1_ex2() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let ans = solve_pt1(input.chars().collect());
        assert_eq!(ans, 6);
    }

    #[test]
    fn it_solves_pt1_ex3() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let ans = solve_pt1(input.chars().collect());
        assert_eq!(ans, 10);
    }

    #[test]
    fn it_solves_pt1_ex4() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let ans = solve_pt1(input.chars().collect());
        assert_eq!(ans, 11);
    }
}
