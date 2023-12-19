use std::collections::HashMap;
use std::{fs, ops, str};

fn main() {
    let input = fs::read_to_string("./data/19.input").unwrap();

    let pt1_ans = solve_pt1(&input);
    println!("Part 1: {}", pt1_ans);

    let pt2_ans = solve_pt2(&input);
    println!("Part 2: {}", pt2_ans);
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Clone, Debug)]
struct PartRange {
    x: ops::Range<i64>,
    m: ops::Range<i64>,
    a: ops::Range<i64>,
    s: ops::Range<i64>,
}

#[derive(Debug)]
struct WorkFlow {
    key: String,
    rules: Vec<WorkRule>,
    default: String,
}

impl WorkFlow {
    #[rustfmt::skip]
    fn apply(&self, p: &Part) -> String {
        for r in self.rules.iter() {
            match [r.category, r.operator] {
                ['x', '<'] => { if p.x < r.val { return r.result.clone() } }
                ['x', '>'] => { if p.x > r.val { return r.result.clone() } }
                ['m', '<'] => { if p.m < r.val { return r.result.clone() } }
                ['m', '>'] => { if p.m > r.val { return r.result.clone() } }
                ['a', '<'] => { if p.a < r.val { return r.result.clone() } }
                ['a', '>'] => { if p.a > r.val { return r.result.clone() } }
                ['s', '<'] => { if p.s < r.val { return r.result.clone() } }
                ['s', '>'] => { if p.s > r.val { return r.result.clone() } }
                _ => unreachable!(),
            }
        }

        self.default.clone()
    }
}

#[derive(Debug)]
struct WorkRule {
    category: char,
    operator: char,
    val: i32,
    result: String,
}

fn solve_pt1(input: &str) -> i32 {
    let (workflows_block, parts_block) = input.split_once("\n\n").unwrap();

    let _ = workflows_block.lines();

    let parts: Vec<Part> = parts_block.lines().filter_map(|p| p.parse().ok()).collect();
    let workflows: Vec<WorkFlow> = workflows_block
        .lines()
        .filter_map(|w| w.parse().ok())
        .collect();

    let mut workflow_map = HashMap::new();

    for w in workflows {
        workflow_map.insert(w.key.clone(), w);
    }

    let mut tot = 0;
    for p in parts {
        let mut res = "in".to_string();
        while res != "A" && res != "R" {
            res = workflow_map.get(&res).unwrap().apply(&p);
        }

        if res == "A" {
            tot += p.x + p.m + p.a + p.s
        }
    }
    tot
}

fn solve_pt2(input: &str) -> i64 {
    let (workflows_block, _) = input.split_once("\n\n").unwrap();

    let _ = workflows_block.lines();

    let workflows: Vec<WorkFlow> = workflows_block
        .lines()
        .filter_map(|w| w.parse().ok())
        .collect();

    let mut workflow_map = HashMap::new();

    for w in workflows {
        workflow_map.insert(w.key.clone(), w);
    }

    let part_range = PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    recurse(&workflow_map, part_range, "in")
}

fn recurse(workflow_map: &HashMap<String, WorkFlow>, part_range: PartRange, key: &str) -> i64 {
    if (part_range.x.end - part_range.x.start) <= 0
        || (part_range.m.end - part_range.m.start) <= 0
        || (part_range.a.end - part_range.a.start) <= 0
        || (part_range.s.end - part_range.s.start) <= 0
    {
        return 0;
    }

    if key == "A" {
        let combinations = (part_range.x.end - part_range.x.start)
            * (part_range.m.end - part_range.m.start)
            * (part_range.a.end - part_range.a.start)
            * (part_range.s.end - part_range.s.start);
        return combinations;
    }

    if key == "R" {
        return 0;
    }

    let workflow = workflow_map.get(key).unwrap();

    let mut tot = 0;
    let mut false_range = part_range.clone();
    for r in workflow.rules.iter() {
        match [r.category, r.operator] {
            ['x', '<'] => {
                let mut true_range = false_range.clone();
                true_range.x.end = r.val as i64;
                false_range.x.start = r.val as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['x', '>'] => {
                let mut true_range = false_range.clone();
                true_range.x.start = (r.val + 1) as i64;
                false_range.x.end = (r.val + 1) as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['m', '<'] => {
                let mut true_range = false_range.clone();
                true_range.m.end = r.val as i64;
                false_range.m.start = r.val as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['m', '>'] => {
                let mut true_range = false_range.clone();
                false_range.m.end = (r.val + 1) as i64;
                true_range.m.start = (r.val + 1) as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['a', '<'] => {
                let mut true_range = false_range.clone();
                true_range.a.end = r.val as i64;
                false_range.a.start = r.val as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['a', '>'] => {
                let mut true_range = false_range.clone();
                true_range.a.start = (r.val + 1) as i64;
                false_range.a.end = (r.val + 1) as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['s', '<'] => {
                let mut true_range = false_range.clone();
                true_range.s.end = r.val as i64;
                false_range.s.start = r.val as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            ['s', '>'] => {
                let mut true_range = false_range.clone();
                true_range.s.start = (r.val + 1) as i64;
                false_range.s.end = (r.val + 1) as i64;
                tot += recurse(workflow_map, true_range, &r.result);
            }
            _ => unreachable!(),
        }
    }

    tot += recurse(workflow_map, false_range, &workflow.default);
    tot
}

struct ParseErr;

impl str::FromStr for Part {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Part, ParseErr> {
        let vals = s.trim_start_matches('{').trim_end_matches('}').split(',');

        let nums: Vec<i32> = vals
            .map(|v| &v[2..])
            .filter_map(|v| v.parse().ok())
            .collect();
        let x = nums[0];
        let m = nums[1];
        let a = nums[2];
        let s = nums[3];
        Ok(Part { x, m, a, s })
    }
}

impl str::FromStr for WorkFlow {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<WorkFlow, ParseErr> {
        let (key, rules_str) = s.trim_end_matches('}').split_once('{').unwrap();

        let mut rules_vec: Vec<&str> = rules_str.split(',').collect();
        let default = rules_vec.pop().unwrap();

        let rules: Vec<WorkRule> = rules_vec
            .iter()
            .map(|r| {
                let (rest, result) = r.split_once(':').unwrap();
                let val: i32 = rest[2..].parse().unwrap();

                let cat_and_op: Vec<char> = rest[..2].chars().collect();

                let category = cat_and_op[0];
                let operator = cat_and_op[1];

                WorkRule {
                    val,
                    result: result.to_string(),
                    category,
                    operator,
                }
            })
            .collect();

        Ok(WorkFlow {
            key: key.to_string(),
            default: default.to_string(),
            rules,
        })
    }
}
