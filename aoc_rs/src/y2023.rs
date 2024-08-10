use crate::{aocerror, AOCError, ProblemPart};

use std::io;

mod d1;

pub fn solve<B: io::BufRead>(day: &str, part: ProblemPart, br: B) -> Result<(), AOCError> {
    match day {
        "1" => d1::solve(part, br),
        _ => Err(aocerror!("invalid day: {}", day)),
    }
}
