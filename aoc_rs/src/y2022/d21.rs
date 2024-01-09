use crate::{aocerror, AOCError, ProblemPart};

use std::collections::HashMap;
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
        }
    }

    fn unapply_left(&self, l_operand: i64, other_side: i64) -> i64 {
        // move l_operand to the other side of the equation:
        // l_operand `self` X = other_side
        match self {
            Operator::Add => other_side - l_operand,
            Operator::Subtract => l_operand - other_side,
            Operator::Multiply => other_side / l_operand,
            Operator::Divide => l_operand / other_side,
        }
    }

    fn unapply_right(&self, r_operand: i64, other_side: i64) -> i64 {
        // move r_operand to the other side of the equation:
        // X `self` r_operand = other_side
        match self {
            Operator::Add => other_side - r_operand,
            Operator::Subtract => other_side + r_operand,
            Operator::Multiply => other_side / r_operand,
            Operator::Divide => other_side * r_operand,
        }
    }
}

#[derive(Debug)]
struct OperationString {
    operator: Operator,
    lhs: String,
    rhs: String,
}

#[derive(Debug)]
enum JobString {
    Number(i64),
    Op(OperationString),
}

#[derive(Debug)]
struct MonkeyString {
    name: String,
    job: JobString,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    lhs: usize,
    rhs: usize,
}

#[derive(Debug)]
enum Job {
    Number(i64),
    Op(Operation),
}

#[derive(Debug)]
struct Monkey {
    parent: Option<usize>,
    job: Job,
}

#[derive(Debug)]
struct MonkeyTree {
    monkeys: Vec<Monkey>,
    root_idx: usize,
    humn_idx: usize,
    result_cache: HashMap<usize, i64>,
}

impl MonkeyTree {
    fn new(monkeys: Vec<MonkeyString>) -> MonkeyTree {
        let mut name_to_index = HashMap::with_capacity(monkeys.len());
        let mut parents = HashMap::with_capacity(monkeys.len());
        let mut monkey_nodes = Vec::with_capacity(monkeys.len());
        let mut root_index = None;
        let mut humn_index = None;
        let mut op_monkeys = 0;
        for (idx, monkey) in monkeys.iter().enumerate() {
            name_to_index.insert(&monkey.name, idx);
            match monkey.name.as_ref() {
                "root" => root_index = Some(idx),
                "humn" => humn_index = Some(idx),
                _ => (),
            }
        }
        for monkey in monkeys.iter() {
            let job_node = match &monkey.job {
                JobString::Number(n) => Job::Number(*n),
                JobString::Op(operation) => {
                    op_monkeys += 1;
                    let lhs_idx = name_to_index.get(&operation.lhs).unwrap();
                    let rhs_idx = name_to_index.get(&operation.rhs).unwrap();
                    let current_monkey_idx = name_to_index.get(&monkey.name).unwrap();
                    assert!(parents.insert(lhs_idx, *current_monkey_idx).is_none());
                    assert!(parents.insert(rhs_idx, *current_monkey_idx).is_none());
                    Job::Op(Operation {
                        operator: operation.operator.clone(),
                        lhs: *lhs_idx,
                        rhs: *rhs_idx,
                    })
                }
            };
            monkey_nodes.push(Monkey {
                parent: None,
                job: job_node,
            });
        }
        for (idx, monkey_node) in monkey_nodes.iter_mut().enumerate() {
            if idx == root_index.unwrap() {
                continue;
            }
            monkey_node.parent = Some(*parents.get(&idx).unwrap());
        }
        MonkeyTree {
            monkeys: monkey_nodes,
            root_idx: root_index.unwrap(),
            humn_idx: humn_index.unwrap(),
            result_cache: HashMap::with_capacity(op_monkeys),
        }
    }

    fn get_monkey_value(&mut self, idx: usize) -> Option<i64> {
        if let Some(val) = self.result_cache.get(&idx) {
            return Some(*val);
        };
        let (lhs_idx, rhs_idx) = match self.monkeys.get(idx) {
            Some(Monkey {
                parent: _,
                job: Job::Op(opnode),
            }) => (opnode.lhs, opnode.rhs),
            Some(Monkey {
                parent: _,
                job: Job::Number(val),
            }) => return Some(*val),
            None => return None,
        };
        let lhs_value = self.get_monkey_value(lhs_idx).unwrap();
        let rhs_value = self.get_monkey_value(rhs_idx).unwrap();
        let result = match self.monkeys.get(idx) {
            Some(Monkey {
                parent: _,
                job: Job::Op(opnode),
            }) => opnode.operator.apply(lhs_value, rhs_value),
            _ => panic!("unreachable"),
        };
        self.result_cache.insert(idx, result);
        Some(result)
    }

    fn path_to_root(&self, idx: usize) -> Vec<usize> {
        let mut path = Vec::new();
        path.push(idx);
        let mut monkey = &self.monkeys[idx];
        while let Some(p_idx) = monkey.parent {
            path.push(p_idx);
            monkey = &self.monkeys[p_idx];
        }
        path
    }
}

impl FromStr for MonkeyString {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, job) = s
            .split_once(':')
            .ok_or_else(|| aocerror!("can't parse monkey: {}", s))?;
        let name_owned = name.to_string();
        if let Ok(num) = job.trim().parse::<i64>() {
            return Ok(MonkeyString {
                name: name_owned,
                job: JobString::Number(num),
            });
        }
        for (ch, enum_value) in [
            ('+', Operator::Add),
            ('-', Operator::Subtract),
            ('*', Operator::Multiply),
            ('/', Operator::Divide),
        ] {
            if let Some((lhs, rhs)) = job.trim().split_once(ch) {
                return Ok(MonkeyString {
                    name: name_owned,
                    job: JobString::Op(OperationString {
                        operator: enum_value,
                        lhs: lhs.trim().to_string(),
                        rhs: rhs.trim().to_string(),
                    }),
                });
            }
        }
        Err(aocerror!("Didn't recognize job: {}", job))
    }
}

fn parse_problem<B: io::BufRead>(br: B) -> Result<MonkeyTree, AOCError> {
    let monkeys: Vec<MonkeyString> = br
        .lines()
        .map(|s| s?.parse::<MonkeyString>())
        .collect::<Result<Vec<MonkeyString>, AOCError>>()?;
    Ok(MonkeyTree::new(monkeys))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "root: pppw + sjmn\
                           \ndbpl: 5\
                           \ncczh: sllz + lgvd\
                           \nzczc: 2\
                           \nptdq: humn - dvpt\
                           \ndvpt: 3\
                           \nlfqf: 4\
                           \nhumn: 5\
                           \nljgn: 2\
                           \nsjmn: drzm * dbpl\
                           \nsllz: 4\
                           \npppw: cczh / lfqf\
                           \nlgvd: ljgn * ptdq\
                           \ndrzm: hmdt - zczc\
                           \nhmdt: 32";

    fn parse_example() -> Result<MonkeyTree, AOCError> {
        let br = io::BufReader::new(EXAMPLE.as_bytes());
        parse_problem(br)
    }

    #[test]
    fn p1_example() {
        let result = parse_example().map(|mt| problem1(mt));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 152);
    }

    #[test]
    fn p2_example() {
        let result = parse_example().map(|mt| problem2(mt));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 301);
    }
}

fn problem1(mut monkey_tree: MonkeyTree) -> i64 {
    monkey_tree.get_monkey_value(monkey_tree.root_idx).unwrap()
}

fn problem2(mut monkey_tree: MonkeyTree) -> i64 {
    let mut humn_to_root = monkey_tree.path_to_root(monkey_tree.humn_idx);
    assert!(!humn_to_root.is_empty());
    let mut accum = None;
    let mut parent_idx = humn_to_root.pop().unwrap();
    while let Some(child_idx) = humn_to_root.pop() {
        let (other_child, operator, path_goes_left) = {
            let parent = &monkey_tree.monkeys[parent_idx];
            match &parent.job {
                Job::Number(_) => panic!("number monkeys can't be parents"),
                Job::Op(op) => {
                    assert!(op.lhs == child_idx || op.rhs == child_idx);
                    let path_goes_left = op.lhs == child_idx;
                    let other_child = if path_goes_left { op.rhs } else { op.lhs };
                    (other_child, op.operator.clone(), path_goes_left)
                }
            }
        };
        let other_value = monkey_tree.get_monkey_value(other_child).unwrap();
        match accum {
            // idx should be root here
            None => {
                assert_eq!(parent_idx, monkey_tree.root_idx);
                accum = Some(other_value)
            }
            Some(a) => {
                accum = Some(if path_goes_left {
                    // X `op` other_value = accum
                    operator.unapply_right(other_value, a)
                } else {
                    operator.unapply_left(other_value, a)
                })
            }
        }
        parent_idx = child_idx;
    }
    accum.unwrap()
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let monkey_tree = parse_problem(br)?;
    let result = match part {
        ProblemPart::P1 => problem1(monkey_tree),
        ProblemPart::P2 => problem2(monkey_tree),
    };
    println!("result: {}", result);
    Ok(())
}
