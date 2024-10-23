use std::fs;

fn main() {
    let infile = fs::read_to_string("./data/06.input").unwrap();

    let ans_pt1 = solve(infile.chars().collect(), 4);
    println!("Part 1: {}", ans_pt1);

    let ans_pt2 = solve(infile.chars().collect(), 14);
    println!("Part 2: {}", ans_pt2);
}

fn solve(datastream_buffer: Vec<char>, n: usize) -> usize {
    let mut ans = 0;
    for (i, w) in datastream_buffer.windows(n).enumerate() {
        let counts: Vec<usize> = (i..(i + n))
            .map(|j| w.iter().filter(|c| *c == &datastream_buffer[j]).count())
            .collect();
        if counts == vec![1; n] {
            ans = i + n;
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
        let ans = solve(input.chars().collect(), 4);
        assert_eq!(ans, 5);
    }

    #[test]
    fn it_solves_pt1_ex2() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let ans = solve(input.chars().collect(), 4);
        assert_eq!(ans, 6);
    }

    #[test]
    fn it_solves_pt1_ex3() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let ans = solve(input.chars().collect(), 4);
        assert_eq!(ans, 10);
    }

    #[test]
    fn it_solves_pt1_ex4() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let ans = solve(input.chars().collect(), 4);
        assert_eq!(ans, 11);
    }

    #[test]
    fn it_solves_pt2_ex1() {
        let input = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let ans = solve(input.chars().collect(), 14);
        assert_eq!(ans, 23);
    }

    #[test]
    fn it_solves_pt2_ex2() {
        let input = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let ans = solve(input.chars().collect(), 14);
        assert_eq!(ans, 23);
    }

    #[test]
    fn it_solves_pt2_ex3() {
        let input = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let ans = solve(input.chars().collect(), 14);
        assert_eq!(ans, 29);
    }

    #[test]
    fn it_solves_pt2_ex4() {
        let input = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        let ans = solve(input.chars().collect(), 14);
        assert_eq!(ans, 26);
    }

    #[test]
    fn it_solves_pt2_ex5() {
        let input = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let ans = solve(input.chars().collect(), 14);
        assert_eq!(ans, 19);
    }
}
