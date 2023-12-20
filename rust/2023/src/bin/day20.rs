use std::boxed::Box;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/20.input").unwrap();

    let pt1_ans = solve_pt1(&input);
    println!("Part 1: {}", pt1_ans);

    let pt2_ans = solve_pt2(&input);
    println!("Part 2: {}", pt2_ans);
}

fn solve_pt1(input: &str) -> i32 {
    let mut comm_mods = construct_modules(input);
    let mut q: VecDeque<Communication> = VecDeque::new();

    let mut low_count = 0;
    let mut high_count = 0;

    for _i in 0..1000 {
        q.push_front(Communication {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(c) = q.pop_back() {
            match c.pulse {
                Pulse::Low => low_count += 1,
                Pulse::High => high_count += 1,
            }

            let m = comm_mods.get_mut(&c.to);

            match m {
                Some(mm) => {
                    let ps = mm.pulse(c);
                    for p in ps {
                        q.push_front(p);
                    }
                }
                None => continue,
            }
        }
    }
    low_count * high_count
}

fn solve_pt2(input: &str) -> i64 {
    let mut comm_mods = construct_modules(input);
    let mut q: VecDeque<Communication> = VecDeque::new();

    let mut lowest_fh = 0;
    let mut lowest_mf = 0;
    let mut lowest_fz = 0;
    let mut lowest_ss = 0;

    for i in 1..100000 {
        q.push_front(Communication {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(c) = q.pop_back() {
            if c.to == "fh" && lowest_fh == 0 && c.pulse == Pulse::Low {
                lowest_fh = i;
            }
            if c.to == "mf" && lowest_mf == 0 && c.pulse == Pulse::Low {
                lowest_mf = i;
            }
            if c.to == "fz" && lowest_fz == 0 && c.pulse == Pulse::Low {
                lowest_fz = i;
            }
            if c.to == "ss" && lowest_ss == 0 && c.pulse == Pulse::Low {
                lowest_ss = i;
            }

            if lowest_fh * lowest_mf * lowest_fz * lowest_ss != 0 {
                break;
            }

            let m = comm_mods.get_mut(&c.to);

            match m {
                Some(mm) => {
                    let ps = mm.pulse(c);
                    for p in ps {
                        q.push_front(p);
                    }
                }
                None => continue,
            }
        }
    }

    // Worked for me. May not work for all.
    lowest_fh * lowest_mf * lowest_fz * lowest_ss
}

trait CommunicationModule {
    fn pulse(&mut self, comm: Communication) -> Vec<Communication>;
}

#[derive(Clone, Debug)]
struct Communication {
    pulse: Pulse,
    from: String,
    to: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug)]
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

fn construct_modules(input: &str) -> HashMap<String, Box<dyn CommunicationModule>> {
    input.lines().fold(HashMap::new(), |mut acc, l| {
        let (from, to_all) = l.split_once(" -> ").unwrap();
        let to = to_all.split(", ").map(|s| s.to_string()).collect();
        let key = from[1..].to_string();
        match from.chars().next().unwrap() {
            '%' => acc.insert(
                key.clone(),
                Box::new(FlipFlopModule { key, on: false, to }) as Box<dyn CommunicationModule>,
            ),
            '&' => {
                let from = input.lines().fold(vec![], |mut acc, l| {
                    let (this_from, this_to) = l.split_once(" -> ").unwrap();
                    let this_from = this_from[1..].to_string();
                    if this_to.contains(&key) {
                        acc.push(Communication {
                            pulse: Pulse::Low,
                            from: this_from,
                            to: key.clone(),
                        });
                    }
                    acc
                });

                acc.insert(
                    key.clone(),
                    Box::new(ConjunctionModule { key, to, from }) as Box<dyn CommunicationModule>,
                )
            }
            'b' => acc.insert(
                "broadcaster".to_string(),
                Box::new(BroadcastModule { to }) as Box<dyn CommunicationModule>,
            ),
            _ => unreachable!(),
        };
        acc
    })
}
