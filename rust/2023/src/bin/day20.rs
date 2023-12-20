use std::collections::{HashMap, VecDeque};
use std::boxed::Box;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/20.example").unwrap();

    // let pt1_ans = solve_pt1(&input);
    // println!("Part 1: {}", pt1_ans);
}

#[derive(Clone)]
enum Pulse {
    High,
    Low,
}

struct Communication {
    pulse: Pulse,
    from: String,
    to: String,
}

trait CommunicationModule {
    fn pulse(&mut self, comm: Communication) -> Vec<Communication>;
}

struct BroadcastModule {
    to: Vec<String>,
}

impl CommunicationModule for BroadcastModule {
    fn pulse(&mut self, _comm: Communication) -> Vec<Communication> {
        self.to
            .iter()
            .map(|to| Communication {
                pulse: Pulse::Low,
                from: String::from("broadcast"),
                to: to.clone(),
            })
            .collect()
    }
}

struct ConjunctionModule {
    key: String,
    from: Vec<Communication>,
    to: Vec<String>,
}

impl CommunicationModule for ConjunctionModule {
    fn pulse(&mut self, comm: Communication) -> Vec<Communication> {
        let from_idx = self.from.iter().position(|c| c.from == comm.from).unwrap();
        self.from[from_idx] = comm;

        let all_high = self.from.iter().all(|c| match c.pulse {
            Pulse::High => true,
            Pulse::Low => false,
        });

        // Then send
        self.to
            .iter()
            .map(|to| Communication {
                pulse: match all_high {
                    true => Pulse::Low,
                    false => Pulse::High,
                },
                from: self.key.clone(),
                to: to.clone(),
            })
            .collect()
    }
}

struct FlipFlopModule {
    key: String,
    to: Vec<String>,
    on: bool,
}

impl CommunicationModule for FlipFlopModule {
    fn pulse(&mut self, comm: Communication) -> Vec<Communication> {
        match (self.on, comm.pulse) {
            (_, Pulse::High) => vec![],
            (false, Pulse::Low) => {
                self.on = true;
                self.to
                    .iter()
                    .map(|t| Communication {
                        pulse: Pulse::High,
                        from: self.key.clone(),
                        to: t.clone(),
                    })
                    .collect()
            }
            (true, Pulse::Low) => {
                self.on = false;
                self.to
                    .iter()
                    .map(|t| Communication {
                        pulse: Pulse::Low,
                        from: self.key.clone(),
                        to: t.clone(),
                    })
                    .collect()
            }
        }
    }
}

fn solve_pt1(input: &str) -> i32 {
    // To build the conjunctions, construct them empty and put in vec, then loop over the
    // constructed items a second time to populate the from communications block.
    
    let modules: Vec<Box<dyn CommunicationModule>> = input.lines().map(|l| {
        let (from, to_all) = l.split_once(" -> ").unwrap();
        let to = to_all.split(", ").map(|s| s.to_string()).collect();
        let key = from[1..].to_string();
        match from.chars().next().unwrap() {
            '%' => Box::new(FlipFlopModule {
                key,
                on: false,
                to,
            }) as Box<dyn CommunicationModule>,
            '&' => Box::new(ConjunctionModule {
                key,
                to,
                from: vec![],
            }) as Box<dyn CommunicationModule>,
            'b' => Box::new(BroadcastModule {
                to
            }) as Box<dyn CommunicationModule>,
            _ => unreachable!(),
        }
    }).collect();

    // let q: VecDeque<Communication> = VecDeque::new();
    // let comm_modules: HashMap<String, Box<dyn CommunicationModule>> = HashMap::new();
    0
}
