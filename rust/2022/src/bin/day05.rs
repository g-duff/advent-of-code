use std::{fs, str};

fn main() {
    #[rustfmt::skip]
    let mut crate_stacks = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P']
    ];

    let infile = load::<RearrangementInstruction>("./data/05.example".to_string());

    infile.into_iter().for_each(|instruction| {
        for _i in 0..instruction.quantity {
            let stacked_item = crate_stacks[(instruction.from - 1) as usize].pop().unwrap();
            crate_stacks[(instruction.to - 1) as usize].push(stacked_item);
        }
    });

    for v in crate_stacks {
        println!("{:?}", v.last().unwrap());
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RearrangementInstruction {
    quantity: i32,
    from: i32,
    to: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRearrangementInstructionError;

impl str::FromStr for RearrangementInstruction {
    type Err = ParseRearrangementInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input_words: Vec<&str> = s.split_whitespace().collect();

        Ok(RearrangementInstruction {
            quantity: input_words[1].parse::<i32>().unwrap(),
            from: input_words[3].parse::<i32>().unwrap(),
            to: input_words[5].parse::<i32>().unwrap(),
        })
    }
}

fn load<T>(path: String) -> Vec<T>
where
    T: str::FromStr,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<T>().ok())
        .collect()
}
