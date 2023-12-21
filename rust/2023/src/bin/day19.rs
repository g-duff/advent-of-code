use std::collections::{HashMap, VecDeque};
use std::{fs, ops, str};

fn main() {
    let input = fs::read_to_string("./data/19.input").unwrap();

    let pt1_ans = solve_pt1(&input);
    println!("Part 1: {}", pt1_ans);

    let pt2_ans_dfs = solve_pt2_dfs(&input);
    println!("Part 2- dfs: {}", pt2_ans_dfs);

    let pt2_ans_bfs = solve_pt2_bfs(&input);
    println!("Part 2- bfs: {}", pt2_ans_bfs);
}

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
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
    val: i64,
    result: String,
}

fn solve_pt1(input: &str) -> i64 {
    let (workflows_block, parts_block) = input.split_once("\n\n").unwrap();

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

fn solve_pt2_bfs(input: &str) -> i64 {
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
    bfs(&workflow_map, part_range, "in")
}

fn bfs(workflow_map: &HashMap<String, WorkFlow>, part_range: PartRange, key: &str) -> i64 {
    let mut q: VecDeque<(&str, PartRange)> = VecDeque::new();

    let mut tot = 0;
    q.push_front((key, part_range));
    while let Some((key, range)) = q.pop_back() {
        if (range.x.end - range.x.start) <= 0
            || (range.m.end - range.m.start) <= 0
            || (range.a.end - range.a.start) <= 0
            || (range.s.end - range.s.start) <= 0
        {
            continue;
        }

        if key == "A" {
            let combinations = (range.x.end - range.x.start)
                * (range.m.end - range.m.start)
                * (range.a.end - range.a.start)
                * (range.s.end - range.s.start);
            tot += combinations;
            continue;
        }

        if key == "R" {
            continue;
        }

        let workflow = workflow_map.get(key).unwrap();
        let mut false_range = range.clone();

        for r in workflow.rules.iter() {
            match [r.category, r.operator] {
                ['x', '<'] => {
                    let mut true_range = false_range.clone();
                    true_range.x.end = r.val;
                    false_range.x.start = r.val;
                    q.push_front((&r.result, true_range));
                }
                ['x', '>'] => {
                    let mut true_range = false_range.clone();
                    true_range.x.start = r.val + 1;
                    false_range.x.end = r.val + 1;
                    q.push_front((&r.result, true_range));
                }
                ['m', '<'] => {
                    let mut true_range = false_range.clone();
                    true_range.m.end = r.val;
                    false_range.m.start = r.val;
                    q.push_front((&r.result, true_range));
                }
                ['m', '>'] => {
                    let mut true_range = false_range.clone();
                    false_range.m.end = r.val + 1;
                    true_range.m.start = r.val + 1;
                    q.push_front((&r.result, true_range));
                }
                ['a', '<'] => {
                    let mut true_range = false_range.clone();
                    true_range.a.end = r.val;
                    false_range.a.start = r.val;
                    q.push_front((&r.result, true_range));
                }
                ['a', '>'] => {
                    let mut true_range = false_range.clone();
                    true_range.a.start = r.val + 1;
                    false_range.a.end = r.val + 1;
                    q.push_front((&r.result, true_range));
                }
                ['s', '<'] => {
                    let mut true_range = false_range.clone();
                    true_range.s.end = r.val;
                    false_range.s.start = r.val;
                    q.push_front((&r.result, true_range));
                }
                ['s', '>'] => {
                    let mut true_range = false_range.clone();
                    true_range.s.start = r.val + 1;
                    false_range.s.end = r.val + 1;
                    q.push_front((&r.result, true_range));
                }
                _ => unreachable!(),
            }
        }
        q.push_front((&workflow.default, false_range));
    }

    tot
}


fn solve_pt2_dfs(input: &str) -> i64 {
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
    dfs(&workflow_map, part_range, "in")
}

fn dfs(workflow_map: &HashMap<String, WorkFlow>, part_range: PartRange, key: &str) -> i64 {
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
                true_range.x.end = r.val;
                false_range.x.start = r.val;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['x', '>'] => {
                let mut true_range = false_range.clone();
                true_range.x.start = r.val + 1;
                false_range.x.end = r.val + 1;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['m', '<'] => {
                let mut true_range = false_range.clone();
                true_range.m.end = r.val;
                false_range.m.start = r.val;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['m', '>'] => {
                let mut true_range = false_range.clone();
                false_range.m.end = r.val + 1;
                true_range.m.start = r.val + 1;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['a', '<'] => {
                let mut true_range = false_range.clone();
                true_range.a.end = r.val;
                false_range.a.start = r.val;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['a', '>'] => {
                let mut true_range = false_range.clone();
                true_range.a.start = r.val + 1;
                false_range.a.end = r.val + 1;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['s', '<'] => {
                let mut true_range = false_range.clone();
                true_range.s.end = r.val;
                false_range.s.start = r.val;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            ['s', '>'] => {
                let mut true_range = false_range.clone();
                true_range.s.start = r.val + 1;
                false_range.s.end = r.val + 1;
                tot += dfs(workflow_map, true_range, &r.result);
            }
            _ => unreachable!(),
        }
    }

    tot += dfs(workflow_map, false_range, &workflow.default);
    tot
}

struct ParseErr;

impl str::FromStr for Part {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Part, ParseErr> {
        let vals = s.trim_start_matches('{').trim_end_matches('}').split(',');

        let nums: Vec<i64> = vals
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
                let val: i64 = rest[2..].parse().unwrap();

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
