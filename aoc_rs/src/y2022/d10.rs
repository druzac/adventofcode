use crate::{aocerror, AOCError, ProblemPart};

use std::collections::VecDeque;
use std::io;
use std::iter;
use std::str::FromStr;

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn iter(&self) -> ProgramIterator {
        let mut deq = VecDeque::new();
        deq.push_back(State::initial());
        ProgramIterator {
            prog: self,
            idx: 0,
            states: deq,
        }
    }
}

enum Instruction {
    Noop,
    Add(i64),
}

impl FromStr for Instruction {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() == 1 && parts[0] == "noop" {
            Ok(Instruction::Noop)
        } else if parts.len() == 2 && parts[0] == "addx" {
            match parts[1].parse::<i64>() {
                Ok(val) => Ok(Instruction::Add(val)),
                Err(_) => Err(aocerror!("failed to parse as integer: {}", parts[1])),
            }
        } else {
            Err(aocerror!("cannot parse line as Instruction: {}", s))
        }
    }
}

struct ProgramIterator<'a> {
    prog: &'a Program,
    idx: usize,
    states: VecDeque<State>,
}

#[derive(Debug)]
struct State {
    cycle: u64,
    register: i64,
}

impl State {
    fn incr_cycle(&self) -> State {
        State {
            cycle: self.cycle + 1,
            register: self.register,
        }
    }

    fn add_val(&self, val: i64) -> State {
        State {
            cycle: self.cycle + 1,
            register: self.register + val,
        }
    }

    fn initial() -> State {
        State {
            cycle: 1,
            register: 1,
        }
    }

    fn signal_strength(&self) -> i64 {
        self.cycle as i64 * self.register
    }
}

impl<'a> Iterator for ProgramIterator<'a> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        match self.states.pop_front() {
            None => None,
            Some(state) => {
                if !self.states.is_empty() || self.idx >= self.prog.instructions.len() {
                    return Some(state);
                }
                match self.prog.instructions[self.idx] {
                    Instruction::Noop => {
                        self.states.push_back(state.incr_cycle());
                    }
                    Instruction::Add(v) => {
                        let next_state = state.incr_cycle();
                        let last_state = next_state.add_val(v);
                        self.states.push_back(next_state);
                        self.states.push_back(last_state);
                    }
                }
                self.idx += 1;
                Some(state)
            }
        }
    }
}

fn is_interesting_cycle(cycle: u64) -> bool {
    cycle >= 20 && (cycle - 20) % 40 == 0
}

fn problem1(prog: Program) -> i64 {
    prog.iter()
        .filter(|state| is_interesting_cycle(state.cycle))
        .map(|state| state.signal_strength())
        .sum()
}

fn problem2(prog: Program) -> i64 {
    let crt_coords = iter::repeat(0..40).take(6).flatten();
    let last_crt_coord = 39;
    for (state, crt_coord) in prog.iter().zip(crt_coords) {
        if (state.register - crt_coord).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if last_crt_coord == crt_coord {
            println!("");
        }
    }
    0
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let instructions: Vec<_> = br
        .lines()
        .map(|s| s.unwrap().parse::<Instruction>().unwrap())
        .collect();
    let program = Program {
        instructions: instructions,
    };
    let result = match part {
        ProblemPart::P1 => problem1(program),
        ProblemPart::P2 => problem2(program),
    };
    println!("result: {}", result);
    Ok(())
}
