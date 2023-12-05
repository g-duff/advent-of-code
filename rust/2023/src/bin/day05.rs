use std::str;

fn main() {}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<i32>,
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
struct Map {}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl str::FromStr for Map {
    type Err = ParseMapError;

    fn from_str(_input: &str) -> Result<Map, ParseMapError> {
        Ok(Map {})
    }
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
        let almanac: Almanac = INPUT.parse().unwrap();

        // Then
        assert_eq!(
            almanac,
            Almanac {
                seeds: vec![79, 14, 55, 13],
                maps: vec![Map {}, Map {}, Map {}, Map {}, Map {}, Map {}, Map {},],
            }
        );
    }
}
