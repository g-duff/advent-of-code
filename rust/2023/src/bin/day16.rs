use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./data/16.input").unwrap();

    let ans_pt1 = solve_pt1(&input);
    println!("Part 1: {}", ans_pt1);

    let ans_pt2 = solve_pt2(&input);
    println!("Part 2 (bit slow): {}", ans_pt2);
}

fn solve_pt1(input: &str) -> usize {
    let contraption_layout: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut activated = vec![vec![0; contraption_layout[0].len()]; contraption_layout.len()];

    let position = [0, 0];
    let direction = [0, 1];
    let mut visited: HashSet<([i32; 2], [i32; 2])> = HashSet::new();

    recurse(
        &mut visited,
        &contraption_layout,
        &mut activated,
        position,
        direction,
    );

    activated
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>()
}

fn solve_pt2(input: &str) -> usize {
    let contraption_layout: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut possibles = vec![];

    for direction in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
        let pos_range = if direction[0] == 0 {
            0..contraption_layout[0].len()
        } else {
            0..contraption_layout.len()
        };

        for p in pos_range {
            let mut activated =
                vec![vec![0; contraption_layout[0].len()]; contraption_layout.len()];

            let position = match direction {
                [0, 1] => [p as i32, 0_i32],
                [0, -1] => [p as i32, contraption_layout[0].len() as i32 - 1],
                [1, 0] => [0_i32, p as i32],
                [-1, 0] => [contraption_layout.len() as i32 - 1, p as i32],
                _ => unreachable!(),
            };

            let mut visited: HashSet<([i32; 2], [i32; 2])> = HashSet::new();

            recurse(
                &mut visited,
                &contraption_layout,
                &mut activated,
                position,
                direction,
            );
            let this_config_ans = activated
                .iter()
                .map(|row| row.iter().sum::<usize>())
                .sum::<usize>();
            possibles.push(this_config_ans);
        }
    }

    *possibles.iter().max().unwrap()
}

fn recurse(
    visited: &mut HashSet<([i32; 2], [i32; 2])>,
    layout: &[Vec<char>],
    activated: &mut [Vec<usize>],
    mut pos: [i32; 2],
    mut dir: [i32; 2],
) {
    while !visited.contains(&(pos, dir))
        && pos[0] >= 0
        && pos[0] <= (layout.len() - 1) as i32
        && pos[1] >= 0
        && pos[1] <= (layout[0].len() - 1) as i32
    {
        activated[pos[0] as usize][pos[1] as usize] = 1;
        visited.insert((pos, dir));

        let current_element = layout[pos[0] as usize][pos[1] as usize];
        match current_element {
            '/' => match dir {
                [0, 1] => dir = [-1, 0],
                [0, -1] => dir = [1, 0],
                [1, 0] => dir = [0, -1],
                [-1, 0] => dir = [0, 1],
                _ => (),
            },
            '\\' => match dir {
                [0, 1] => dir = [1, 0],
                [0, -1] => dir = [-1, 0],
                [1, 0] => dir = [0, 1],
                [-1, 0] => dir = [0, -1],
                _ => (),
            },
            '-' => match dir {
                [1, 0] => {
                    recurse(visited, layout, activated, pos, [0, -1]);
                    recurse(visited, layout, activated, pos, [0, 1]);
                    break;
                }
                [-1, 0] => {
                    recurse(visited, layout, activated, pos, [0, -1]);
                    recurse(visited, layout, activated, pos, [0, 1]);
                    break;
                }
                _ => (),
            },
            '|' => match dir {
                [0, 1] => {
                    recurse(visited, layout, activated, pos, [1, 0]);
                    recurse(visited, layout, activated, pos, [-1, 0]);
                    break;
                }
                [0, -1] => {
                    recurse(visited, layout, activated, pos, [1, 0]);
                    recurse(visited, layout, activated, pos, [-1, 0]);
                    break;
                }
                _ => (),
            },
            _ => (),
        }

        //         for row in &mut *activated {
        //             println!(
        //                 "{}",
        //                 row.iter()
        //                     .map(|c| match c {
        //                         0 => '.',
        //                         1 => '#',
        //                         _ => unreachable!(),
        //                     })
        //                     .collect::<String>()
        //             );
        //         }
        //         println!("{:?}", pos);
        //         println!();

        pos[0] += dir[0];
        pos[1] += dir[1];
    }
}
