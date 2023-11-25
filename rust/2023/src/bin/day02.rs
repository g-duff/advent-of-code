use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    pt1();
    pt2();
}

fn pt1() {
    let file = File::open("./data/02.input").unwrap();
    let their_possible_moves = "ABC";
    let my_possible_moves = "XYZ";

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let my_move_score = my_possible_moves.find(moves[1]).unwrap() + 1;
            let their_move_score = their_possible_moves.find(moves[0]).unwrap() + 1;
            let my_win_bonus = 3 * (2 - (4 + their_move_score - my_move_score) % 3);

            my_total_score + my_move_score + my_win_bonus
        });

    println!("Part 1: {}", total_score);
}

fn pt2() {
    let file = File::open("./data/02.input").unwrap();
    let their_possible_moves = "ABC";
    let win_scale_factors = "XYZ";

    let total_score = BufReader::new(file)
        .lines()
        .fold(0, |my_total_score, line| {
            let var = line.unwrap();
            let moves: Vec<&str> = var.split(" ").collect();

            let their_move_score = their_possible_moves.find(moves[0]).unwrap() + 1;
            let win_scale_factor = win_scale_factors.find(moves[1]).unwrap();
            let my_move_score = (their_move_score + win_scale_factor + 1) % 3 + 1;

            my_total_score + my_move_score + win_scale_factor * 3
        });
    println!("Part 2: {}", total_score);
}
