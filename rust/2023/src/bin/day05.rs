use std::{fs, ops, str};

fn main() {
    let almanac: Almanac = fs::read_to_string("./data/05.input")
        .unwrap()
        .parse()
        .unwrap();

    // let pt1_answer = solve_pt1(almanac);
    // println!("{pt1_answer}");

    let pt2_answer = solve_pt2(almanac);
    println!("{pt2_answer}");
}

fn solve_pt1(almanac: Almanac) -> i64 {
    let seeds = almanac.seeds;

    let answer: Vec<i64> = seeds
        .iter()
        .map(|s| {
            let mut a = s.clone();
            for m in &almanac.maps {
                for t in &m.transforms {
                    if t.source_range.contains(&a) {
                        a = a + t.offset;
                        break;
                    }
                }
            }
            a
        })
        .collect();

    let minval = answer.iter().min().unwrap();
    *minval
}

fn solve_pt2(almanac: Almanac) -> i64 {

    let seeds: Vec<i64> = almanac
        .seeds
        .chunks(2)
        .map(|s| (s[0]..s[0]+s[1]).collect::<Vec<i64>>())
        .flatten().collect();

    let answer: Vec<i64> = seeds
        .iter()
        .map(|s| {
            let mut a = s.clone();
            for m in &almanac.maps {
                for t in &m.transforms {
                    if t.source_range.contains(&a) {
                        a = a + t.offset;
                        break;
                    }
                }
            }
            a
        })
        .collect();

    let minval = answer.iter().min().unwrap();
    *minval
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseAlmanacError;

impl str::FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(input: &str) -> Result<Almanac, ParseAlmanacError> {
        let mut split_input = input.split("\n\n");

        let seeds_line = split_input.next().unwrap();

        let (_, seed_numbers) = seeds_line.split_once(' ').unwrap();

        let seeds = seed_numbers
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        let maps = split_input.map(|m| m.parse().unwrap()).collect();

        Ok(Almanac { seeds, maps })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    label: String,
    transforms: Vec<Transform>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl str::FromStr for Map {
    type Err = ParseMapError;

    fn from_str(input: &str) -> Result<Map, ParseMapError> {
        let mut lines = input.lines();

        let label = lines.next().unwrap().to_string();

        let transforms = input
            .lines()
            .skip(1)
            .map(|t| t.split(' ').map(|i| i.parse::<i64>().unwrap()).collect())
            .map(|t: Vec<i64>| Transform {
                source_range: ops::Range {
                    start: t[1],
                    end: t[1] + t[2],
                },
                offset: t[0] - t[1],
            })
            .collect();
        Ok(Map { label, transforms })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Transform {
    source_range: ops::Range<i64>,
    offset: i64,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parses_input() {
        // Given
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

        let almanac: Almanac = input.parse().unwrap();

        // Then
        assert_eq!(
            almanac,
            Almanac {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    Map {
                        label: "seed-to-soil map:".to_string(),
                        transforms: vec![
                            Transform {
                                source_range: ops::Range {
                                    start: 98,
                                    end: 100,
                                },
                                offset: -48,
                            },
                            Transform {
                                source_range: ops::Range { start: 50, end: 98 },
                                offset: 2,
                            },
                        ],
                    },
                    Map {
                        label: "soil-to-fertilizer map:".to_string(),
                        transforms: vec![
                            Transform {
                                source_range: ops::Range { start: 15, end: 52 },
                                offset: -15
                            },
                            Transform {
                                source_range: ops::Range { start: 52, end: 54 },
                                offset: -15
                            },
                            Transform {
                                source_range: ops::Range { start: 0, end: 15 },
                                offset: 39
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn solves_pt1() {
        // Given
        let input = INPUT.parse().unwrap();

        // When
        let answer = solve_pt1(input);

        // Then
        assert_eq!(answer, 35)
    }

    #[test]
    fn solves_pt2() {
        // Given
        let input = INPUT.parse().unwrap();

        // When
        let answer = solve_pt2(input);

        // Then
        assert_eq!(answer, 46)
    }
}
