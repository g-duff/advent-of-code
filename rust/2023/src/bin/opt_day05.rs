use std::str;

fn main() {
    let mut foo = Layer {
        label: String::from("foo"),
        maps: vec![],
    };

    let bar = Layer {
        label: String::from("bar"),
        maps: vec![],
    };

    let baz = Layer {
        label: String::from("baz"),
        maps: vec![],
    };

    foo.combine_with(bar);
    foo.combine_with(baz);
}

// Combine transformation ranges into "super" transformation
// Test against problem 1
// For pt2, find minimum out of "from" - to get the minimum value
// fn combine_maps(conversions: Vec<Conversion>) -> Vec<TransformMarker> {
// }

#[derive(Debug, PartialEq, Eq)]
struct Layer {
    label: String,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    from: i64,
    to: i64,
    shift: i64,
}

impl Layer {
    fn combine_with(&mut self, layer: Layer) {
        let mut new_transforms: Vec<Map> = vec![];
        self.maps.iter().for_each(|m| {
            // If within a transform range, split and apply transform
            // else clone or move
            new_transforms.push(Map {
                from: m.from,
                to: m.to,
                shift: m.shift,
            });
        });

        self.maps = new_transforms;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl str::FromStr for Layer {
    type Err = ParseMapError;

    fn from_str(input: &str) -> Result<Layer, ParseMapError> {
        let mut lines = input.split('\n');
        let label = lines.next().unwrap().to_string();

        let transforms = input
            .lines()
            .skip(1)
            .map(|t| t.split(' ').map(|i| i.parse::<i64>().unwrap()).collect())
            .map(|t: Vec<i64>| Map {
                from: t[1],
                to: t[1] + t[2],
                shift: t[0] - t[1],
            })
            .collect();
        Ok(Layer {
            label,
            maps: transforms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_maps() {
        // Given
        let input = "seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

        let maps: Vec<Layer> = input.split("\n\n").map(|i| i.parse().unwrap()).collect();

        // Then
        assert_eq!(
            maps,
            vec![
                Layer {
                    label: "seed-to-soil map:".to_string(),
                    maps: vec![
                        Map {
                            from: 98,
                            to: 100,
                            shift: -48,
                        },
                        Map {
                            from: 50,
                            to: 98,
                            shift: 2,
                        },
                    ],
                },
                Layer {
                    label: "soil-to-fertilizer map:".to_string(),
                    maps: vec![
                        Map {
                            from: 15,
                            to: 52,
                            shift: -15,
                        },
                        Map {
                            from: 52,
                            to: 54,
                            shift: -15,
                        },
                        Map {
                            from: 0,
                            to: 15,
                            shift: 39,
                        }
                    ]
                }
            ]
        );
    }
}
