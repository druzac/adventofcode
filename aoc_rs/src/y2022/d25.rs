use crate::{aocerror, AOCError, ProblemPart};

use std::fmt;
use std::io;
use std::str::FromStr;

enum SnafuDigit {
    MinusTwo,
    MinusOne,
    Zero,
    One,
    Two,
}

impl SnafuDigit {
    fn from_char(c: char) -> Result<Self, AOCError> {
        match c {
            '1' => Ok(SnafuDigit::One),
            '2' => Ok(SnafuDigit::Two),
            '0' => Ok(SnafuDigit::Zero),
            '-' => Ok(SnafuDigit::MinusOne),
            '=' => Ok(SnafuDigit::MinusTwo),
            _ => Err(aocerror!("invalid character: {}", c)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            SnafuDigit::MinusTwo => '=',
            SnafuDigit::MinusOne => '-',
            SnafuDigit::Zero => '0',
            SnafuDigit::One => '1',
            SnafuDigit::Two => '2',
        }
    }

    fn to_value(&self) -> i64 {
        match self {
            SnafuDigit::MinusTwo => -2,
            SnafuDigit::MinusOne => -1,
            SnafuDigit::Zero => 0,
            SnafuDigit::One => 1,
            SnafuDigit::Two => 2,
        }
    }
}

impl fmt::Display for SnafuDigit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

struct Snafu {
    // first element of the vec is the least significant digit.
    rep: Vec<SnafuDigit>,
}

impl Snafu {
    fn digits_least_sig_to_most_sig<'a>(&'a self) -> impl Iterator<Item = &'a SnafuDigit> + 'a {
        self.rep.iter()
    }

    fn digits_most_sig_to_least_sig<'a>(&'a self) -> impl Iterator<Item = &'a SnafuDigit> + 'a {
        self.rep.iter().rev()
    }

    fn to_snafu_helper(n: u64, mut accum: Vec<SnafuDigit>) -> Snafu {
        let mut q = n / 5;
        let r = n % 5;
        let c = match r {
            0 => SnafuDigit::Zero,
            1 => SnafuDigit::One,
            2 => SnafuDigit::Two,
            3 => {
                q += 1;
                SnafuDigit::MinusTwo
            }
            4 => {
                q += 1;
                SnafuDigit::MinusOne
            }
            _ => panic!("unreachable"),
        };
        accum.push(c);
        if q > 0 {
            Snafu::to_snafu_helper(q, accum)
        } else {
            Snafu { rep: accum }
        }
    }
}

impl FromStr for Snafu {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s
            .chars()
            .map(SnafuDigit::from_char)
            .collect::<Result<Vec<SnafuDigit>, AOCError>>()?;
        v.reverse();
        Ok(Snafu { rep: v })
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.digits_most_sig_to_least_sig()
            .map(|d| d.fmt(f))
            .collect::<Result<(), fmt::Error>>()
    }
}

impl From<u64> for Snafu {
    fn from(value: u64) -> Snafu {
        let v = Vec::new();
        Snafu::to_snafu_helper(value, v)
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> i64 {
        value
            .digits_least_sig_to_most_sig()
            .enumerate()
            .map(|(place, val)| 5_i64.pow(place as u32) * val.to_value())
            .sum()
    }
}

pub fn solve_inner<B: io::BufRead>(part: ProblemPart, br: B) -> Result<String, AOCError> {
    let mut snafus = Vec::new();
    for line in br.lines() {
        snafus.push(line?.parse::<Snafu>()?);
    }
    match part {
        ProblemPart::P1 => {
            let total = snafus.into_iter().map(i64::from).sum::<i64>();
            Ok(format!("{}", Snafu::from(total as u64)))
        }
        ProblemPart::P2 => panic!("unimplemented"),
    }
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let result = solve_inner(part, br)?;
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::From;
    use std::io::BufRead;

    #[test]
    fn first_from_example() {
        let s = "1=-0-2";
        let snafu = s.parse::<Snafu>().unwrap();
        assert_eq!(i64::from(snafu), 1747);
    }

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    fn get_example_br() -> impl io::BufRead {
        io::BufReader::new(EXAMPLE.as_bytes())
    }

    #[test]
    fn example() {
        let result = solve_inner(ProblemPart::P1, get_example_br());
        assert_eq!(result.unwrap(), "2=-1=0");
    }
}
