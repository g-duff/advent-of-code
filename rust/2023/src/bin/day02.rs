use std::fs::File;
use std::io::{BufRead, BufReader};

const THEIR_POSSIBLE_MOVES: &str = "ABC";
const MY_POSSIBLE_MOVES: &str = "XYZ";
const WIN_SCALE_FACTORS: &str = "XYZ";

// Row: opponent rock, paper, scisors
// Col: me rock, paper, scisors
#[rustfmt::skip]
const PT1_ARR: [[i32; 3]; 3] = [
    [1 + 3, 2 + 6,  3       ],
    [1,     2 + 3,  3 + 6   ],
    [1 + 6, 2,      3 + 3   ],
];

// Row: opponent rock, paper, scisors
// Col: me lose, draw, win
#[rustfmt::skip]
const PT2_ARR: [[i32; 3]; 3] = [
    [3, 1 + 3, 2 + 6],
    [1, 2 + 3, 3 + 6],
    [2, 3 + 3, 1 + 6],
];

fn main() {
    pt1();
    pt2();

    pt1fast();
    pt2fast();
}

fn pt1() {
    let file = File::open("./data/02.input").unwrap();

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let my_move_score = MY_POSSIBLE_MOVES.find(moves[1]).unwrap() + 1;
            let their_move_score = THEIR_POSSIBLE_MOVES.find(moves[0]).unwrap() + 1;
            let my_win_bonus = 3 * (2 - (4 + their_move_score - my_move_score) % 3);

            my_total_score + my_move_score + my_win_bonus
        });

    println!("Part 1: {}", total_score);
}

fn pt2() {
    let file = File::open("./data/02.input").unwrap();

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let their_move_score = THEIR_POSSIBLE_MOVES.find(moves[0]).unwrap() + 1;
            let win_scale_factor = WIN_SCALE_FACTORS.find(moves[1]).unwrap();
            let my_move_score = (their_move_score + win_scale_factor + 1) % 3 + 1;

            my_total_score + my_move_score + win_scale_factor * 3
        });
    println!("Part 2: {}", total_score);
}

fn pt1fast() {
    let file = File::open("./data/02.input").unwrap();

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let my_move_score = MY_POSSIBLE_MOVES.find(moves[1]).unwrap();
            let their_move_score = THEIR_POSSIBLE_MOVES.find(moves[0]).unwrap();

            my_total_score + PT1_ARR[their_move_score][my_move_score]
        });

    println!("Part 1 fast: {}", total_score);
}

fn pt2fast() {
    let file = File::open("./data/02.input").unwrap();

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let their_move_score = THEIR_POSSIBLE_MOVES.find(moves[0]).unwrap();
            let win_scale_factor = WIN_SCALE_FACTORS.find(moves[1]).unwrap();

            my_total_score + PT2_ARR[their_move_score][win_scale_factor]
        });
    println!("Part 2 fast: {}", total_score);
}
