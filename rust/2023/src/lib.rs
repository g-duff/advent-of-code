use std::{fs, str};

pub fn load_to_vec<T>(path: &str) -> Vec<T>
where
    T: str::FromStr,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<T>().ok())
        .collect()
}
