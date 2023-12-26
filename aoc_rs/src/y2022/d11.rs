use crate::{AOCError, ProblemPart};

use std::collections::VecDeque;
use std::io;

struct DivisorTest {
    divisor: u64,
    true_destination: usize,
    false_destination: usize,
}

impl DivisorTest {
    fn test(&self, n: u64) -> bool {
        n % self.divisor == 0
    }

    fn destination_monkey(&self, n: u64) -> usize {
        if self.test(n) {
            self.true_destination
        } else {
            self.false_destination
        }
    }
}

enum Operator {
    Add,
    Multiply,
}

struct Operation {
    operator: Operator,
    operand: Option<u64>,
}

impl Operation {
    fn apply(&self, n: u64) -> u64 {
        let operand = match self.operand {
            Some(m) => m,
            None => n,
        };
        match self.operator {
            Operator::Add => n + operand,
            Operator::Multiply => n * operand,
        }
    }
}

struct Monkey {
    items: VecDeque<u64>,
    inspect_op: Operation,
    test: DivisorTest,
}

impl Monkey {
    fn process_turn(&mut self, decrease_worry: bool) -> Vec<(u64, usize)> {
        let mut items_and_destinations = Vec::with_capacity(self.items.len());
        for item in &self.items {
            // inspect item, increase worry level
            let mut worry_level = self.inspect_op.apply(*item);
            // finish inspection, decrease worry level
            if decrease_worry {
                worry_level = worry_level / 3;
            }
            let dest_monkey_idx = self.test.destination_monkey(worry_level);
            items_and_destinations.push((worry_level, dest_monkey_idx));
        }
        self.items.truncate(0);
        items_and_destinations
    }

    fn accept_item(&mut self, n: u64) {
        self.items.push_back(n);
    }

    fn num_items(&self) -> usize {
        self.items.len()
    }
}

fn parse_items(item_str: &str) -> VecDeque<u64> {
    let item_line: Vec<_> = item_str.trim().split(' ').collect();
    assert!(item_line[0] == "Starting");
    assert!(item_line[1] == "items:");
    let mut items = VecDeque::new();
    for item_str in item_line.iter().skip(2) {
        items.push_back(item_str.trim_end_matches(',').parse::<u64>().unwrap());
    }
    items
}

fn parse_operation(op_line: &str) -> Operation {
    let op_words: Vec<_> = op_line.trim().split(' ').collect();
    assert!(op_words[0] == "Operation:");
    assert!(op_words[1] == "new");
    assert!(op_words[2] == "=");
    assert!(op_words[3] == "old");
    let operator = match op_words[4] {
        "*" => Operator::Multiply,
        "+" => Operator::Add,
        _ => panic!(),
    };
    let operand = match op_words[5] {
        "old" => None,
        x => Some(x.parse::<u64>().unwrap()),
    };
    Operation {
        operator: operator,
        operand: operand,
    }
}

fn parse_test(tline1: &str, tline2: &str, tline3: &str) -> DivisorTest {
    let line1_words: Vec<_> = tline1.trim().split(' ').collect();
    assert!(line1_words[0] == "Test:");
    assert!(line1_words[1] == "divisible");
    assert!(line1_words[2] == "by");
    let divisor = line1_words[3].parse::<u64>().unwrap();
    let line2_words: Vec<_> = tline2.trim().split(' ').collect();
    assert!(line2_words[0..5] == vec!["If", "true:", "throw", "to", "monkey"]);
    let true_destination = line2_words[5].parse::<usize>().unwrap();
    let line3_words: Vec<_> = tline3.trim().split(' ').collect();
    assert!(line3_words[0..5] == vec!["If", "false:", "throw", "to", "monkey"]);
    let false_destination = line3_words[5].parse::<usize>().unwrap();
    DivisorTest {
        divisor: divisor,
        true_destination: true_destination,
        false_destination: false_destination,
    }
}

fn parse_monkey(rest: &[String]) -> (Option<Monkey>, &[String]) {
    if rest.is_empty() {
        return (None, rest);
    }
    let monkey_line: Vec<_> = rest[0].split(' ').collect();
    assert!(monkey_line.len() == 2);
    assert!(monkey_line[0] == "Monkey");
    let monkey = Monkey {
        items: parse_items(&rest[1]),
        inspect_op: parse_operation(&rest[2]),
        test: parse_test(&rest[3], &rest[4], &rest[5]),
    };
    (Some(monkey), &rest[6..])
}

fn parse(lines: Vec<String>) -> Vec<Monkey> {
    let mut rest = &lines[0..];
    let mut monkeys = Vec::new();
    let mut idx = 0;
    while rest.len() > 0 {
        assert!(rest[0].split(' ').skip(1).next() == Some(&format!("{}:", idx)));
        let result = parse_monkey(rest);
        rest = if result.1.is_empty() {
            result.1
        } else {
            &result.1[1..]
        };
        monkeys.push(result.0.unwrap());
        idx += 1;
    }
    monkeys
}

fn run_simulation(
    mut monkeys: Vec<Monkey>,
    decrease_worry: bool,
    modulo: Option<u64>,
    num_rounds: usize,
) -> u64 {
    let mut items_inspected_count = vec![0; monkeys.len()];
    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            items_inspected_count[i] += monkeys[i].num_items();
            let thrown_items = monkeys[i].process_turn(decrease_worry);
            for (item, destination_idx) in thrown_items {
                monkeys[destination_idx].accept_item(modulo.map_or(item, |x| item % x));
            }
        }
    }
    items_inspected_count.sort_by(|a, b| b.cmp(a));
    (items_inspected_count[0] * items_inspected_count[1]) as u64
}

fn problem1(monkeys: Vec<Monkey>) -> u64 {
    run_simulation(monkeys, true, None, 20)
}

fn problem2(monkeys: Vec<Monkey>) -> u64 {
    let prod = monkeys
        .iter()
        .map(|m| m.test.divisor)
        .fold(1, |accum, div| accum * div);
    run_simulation(monkeys, false, Some(prod), 10000)
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let lines: Vec<_> = br.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let monkeys = parse(lines);
    let result = match part {
        ProblemPart::P1 => problem1(monkeys),
        ProblemPart::P2 => problem2(monkeys),
    };
    println!("result: {}", result);
    Ok(())
}
