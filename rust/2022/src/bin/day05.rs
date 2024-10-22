use std::fs;

fn main() {
    let infile = fs::read_to_string("./data/05.example").unwrap();
    let my_inf: Vec<String> = infile.split("\n\n").map(|s| s.to_string()).collect();

    let stacks = &my_inf[0];
    let parsed_stacks = parse_stacks(stacks);

    let instructions = &my_inf[1];
    let parsed_instructions = parse_instructions(instructions);

    let pt1_ans = solve_pt1(parsed_stacks, parsed_instructions);
    println!("Part 1: {pt1_ans}");
}

fn solve_pt1(mut stacks: Vec<Vec<char>>, instructions: Vec<Vec<usize>>) -> String {
    for instruction in instructions {
        let n_crates = instruction[0];
        let from_stack = instruction[1] - 1;
        let to_stack = instruction[2] - 1;

        for _i in 0..n_crates {
            let aoc_crate = stacks[from_stack].pop().expect("Oh no");
            stacks[to_stack].push(aoc_crate);
        }
    }

    stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect()
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut stacks_grid: Vec<Vec<char>> = stacks.split('\n').map(|l| l.chars().collect()).collect();
    stacks_grid.reverse();

    let stacks_height = stacks_grid.len();

    let n_stacks = (stacks_grid[0].len() + 1) / 4;

    let mut to_return_stacks: Vec<Vec<char>> = Vec::with_capacity(n_stacks);

    for _i in 0..n_stacks {
        to_return_stacks.push(Vec::with_capacity(stacks_height));
    }

    for row in &stacks_grid[1..stacks_grid.len()] {
        for c_idx in 0..(n_stacks) {
            let this_char = row[c_idx * 4 + 1];
            if this_char != ' ' {
                to_return_stacks[c_idx].push(this_char);
            }
        }
    }

    to_return_stacks
}

fn parse_instructions(move_instructions: &str) -> Vec<Vec<usize>> {
    let mut mid: Vec<&str> = move_instructions.split('\n').collect();

    // Poor man's filter
    mid = mid[0..mid.len() - 1].to_vec();

    mid.iter()
        .map(|l| {
            let ns: Vec<&str> = l.split(' ').collect();
            vec![
                ns[1].parse::<usize>().unwrap(),
                ns[3].parse::<usize>().unwrap(),
                ns[5].parse::<usize>().unwrap(),
            ]
        })
        .collect()
}
