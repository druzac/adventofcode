use crate::{aocerror, AOCError, ProblemPart};

use std::io;

mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

pub fn solve<B: io::BufRead>(day: &str, part: ProblemPart, br: B) -> Result<(), AOCError> {
    match day {
        "5" => d5::solve(part, br),
        "6" => d6::solve(part, br),
        "7" => d7::solve(part, br),
        "8" => d8::solve(part, br),
        "9" => d9::solve(part, br),
        "10" => d10::solve(part, br),
        "11" => d11::solve(part, br),
        "12" => d12::solve(part, br),
        "13" => d13::solve(part, br),
        "14" => d14::solve(part, br),
        "15" => d15::solve(part, br),
        "16" => d16::solve(part, br),
        "17" => d17::solve(part, br),
        "18" => d18::solve(part, br),
        "19" => d19::solve(part, br),
        "20" => d20::solve(part, br),
        "21" => d21::solve(part, br),
        "22" => d22::solve(part, br),
        "23" => d23::solve(part, br),
        "24" => d24::solve(part, br),
        _ => Err(aocerror!("invalid day: {}", day)),
    }
}
