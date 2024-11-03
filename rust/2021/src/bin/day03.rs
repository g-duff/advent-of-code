use std::fs;

fn main() {
    let input = &fs::read_to_string("./data/day03.input").unwrap();

    let parsed_input = input.lines().map(|l| l.chars().map(|c| match c {
        '1' => 1,
        _ => 0,
    }).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    let pt2_ans = solve_pt2(parsed_input);
    println!("Part 2: {pt2_ans}");
}

fn solve_pt2(input: Vec<Vec<i32>>) -> i32 {
    let n_cols = input[0].len();
    let mut col_idx = 0;

    let mut oxygen_data = input.clone();
    while oxygen_data.len() > 1 && col_idx < n_cols {
        let n_rows = oxygen_data.len();
        let set_bits = oxygen_data.iter().filter(|row| row[col_idx] == 1).count();
        let most_common_bit = ((set_bits * 2) >= n_rows) as i32;
        oxygen_data.retain(|row| row[col_idx] == most_common_bit);
        col_idx += 1
    }

    let ox_str_radix: String = oxygen_data[0].iter().map(|c| match c {
        0 => '0',
        _ => '1',
    }).collect();
    let ox_rating = i32::from_str_radix(&ox_str_radix, 2).unwrap();

    col_idx = 0;

    let mut co2_data = input;
    while co2_data.len() > 1 && col_idx < n_cols {
        let set_bits = co2_data.iter().filter(|row| row[col_idx] == 1).count();
        let least_common_bit = ((set_bits * 2) < co2_data.len()) as i32;
        co2_data.retain(|row| row[col_idx] == least_common_bit);
        col_idx += 1;
    }

    let co2_str_radix: String = co2_data[0].iter().map(|c| match c {
        0 => '0',
        _ => '1',
    }).collect();
    let co2_rating = i32::from_str_radix(&co2_str_radix, 2).unwrap();

    co2_rating * ox_rating
}
