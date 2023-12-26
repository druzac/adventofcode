use crate::{AOCError, ProblemPart};

use std::io;
use std::io::BufRead;

// not sure what the types should be.
fn parse_problem<B: BufRead>(_: io::Lines<B>) -> Result<Vec<String>, AOCError> {
    panic!("unimplemented")
}

fn problem1(_: Vec<String>) -> u64 {
    panic!("unimplemented")
}

fn problem2(_: Vec<String>) -> u64 {
    panic!("unimplemented")
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let problem = parse_problem(br.lines())?;
    let result = match part {
        ProblemPart::P1 => problem1(problem),
        ProblemPart::P2 => problem2(problem),
    };
    println!("result: {}", result);
    Ok(())
}
