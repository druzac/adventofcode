mod y2022;

use std::env;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::path;
use std::str::FromStr;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AOCError {
    file: &'static str,
    line: u32,
    error_kind: AOCErrorKind,
}

#[derive(Debug)]
pub enum AOCErrorKind {
    IOError(io::Error),
    ParseError(AOCParseErrorKind),
    Other(String),
}

#[derive(Debug)]
pub enum AOCParseErrorKind {
    Number(ParseIntError),
}

impl AOCError {
    fn new_generic(file: &'static str, line: u32, msg: String) -> AOCError {
        AOCError {
            file: file,
            line: line,
            error_kind: AOCErrorKind::Other(msg),
        }
    }
}

impl From<ParseIntError> for AOCError {
    fn from(e: ParseIntError) -> AOCError {
        AOCError {
            file: file!(),
            line: line!(),
            error_kind: AOCErrorKind::ParseError(AOCParseErrorKind::Number(e)),
        }
    }
}

impl From<io::Error> for AOCError {
    fn from(e: io::Error) -> AOCError {
        AOCError {
            file: file!(),
            line: line!(),
            error_kind: AOCErrorKind::IOError(e),
        }
    }
}

macro_rules! aocerror {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        AOCError::new_generic(file!(), line!(), msg)
    }}
}

#[derive(PartialEq, Eq, Debug)]
pub enum ProblemPart {
    P1,
    P2,
}

impl FromStr for ProblemPart {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p1" => Ok(ProblemPart::P1),
            "p2" => Ok(ProblemPart::P2),
            _ => Err(aocerror!("invalid problem part: {}", s)),
        }
    }
}

pub(crate) use aocerror;

fn main() -> Result<(), AOCError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("usage: {} year day part problem_dir file_name", args[0]);
        return Err(aocerror!("bad command line: {:?}", args));
    }
    let year = &args[1];
    let day = &args[2];
    let part = args[3].parse::<ProblemPart>()?;
    let mut input_path = path::PathBuf::from(&args[4]);
    input_path.push(&year);
    input_path.push(&day);
    input_path.push(&args[5]);
    let br = io::BufReader::new(fs::File::open(&input_path)?);
    match args[1].as_str() {
        "2022" => y2022::solve(day, part, br),
        _ => panic!(),
    }
}
