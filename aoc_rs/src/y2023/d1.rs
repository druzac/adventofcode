use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::io;

fn part_one<B: io::BufRead>(br: B) -> Result<i64, AOCError> {
    let mut sum = 0;
    for maybe_line in br.lines() {
        let line = maybe_line?;
        let mut digits_it = line.chars().filter(|c| c.is_ascii_digit());
        let mut digits = String::new();
        match digits_it.next() {
            Some(d) => digits.push(d),
            None => return Err(aocerror!("couldn't find a digit in line: {}", line)),
        }
        match digits_it.last() {
            Some(d) => digits.push(d),
            None => digits.push(digits.chars().next().unwrap()),
        }
        sum += digits.parse::<i64>()?;
    }
    Ok(sum)
}

// rust doesn't have regexes in the standard lib. instead of importing
// a crate we can do this with the str find API.
fn find_by<F, G, H>(cmp: F, searcher1: G, searcher2: H) -> Option<char>
where
    F: Fn((usize, char), (usize, char)) -> (usize, char),
    G: Fn(&str) -> Option<usize>,
    H: Fn(char) -> Option<usize>,
{
    let digit_items = [
        ('0', '0'),
        ('1', '1'),
        ('2', '2'),
        ('3', '3'),
        ('4', '4'),
        ('5', '5'),
        ('6', '6'),
        ('7', '7'),
        ('8', '8'),
        ('9', '9'),
    ];
    let mut current = None;
    for (val, target) in digit_items {
        match (current, searcher2(target)) {
            (Some(existing), Some(idx)) => current = Some(cmp(existing, (idx, val))),
            (None, Some(idx)) => current = Some((idx, val)),
            _ => (),
        }
    }
    let word_items = [
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ];
    for (val, target) in word_items {
        match (current, searcher1(target)) {
            (Some(existing), Some(idx)) => current = Some(cmp(existing, (idx, val))),
            (None, Some(idx)) => current = Some((idx, val)),
            _ => (),
        }
    }
    current.map(|pair| pair.1)
}

fn part_two<B: io::BufRead>(br: B) -> Result<i64, AOCError> {
    let mut sum = 0;
    for maybe_line in br.lines() {
        let line = maybe_line?;
        let error_lambda = || aocerror!("can't find number in line: {}", line);
        let first =
            find_by(cmp::min, |pat| line.find(pat), |c| line.find(c)).ok_or_else(error_lambda)?;
        let last =
            find_by(cmp::max, |pat| line.rfind(pat), |c| line.rfind(c)).ok_or_else(error_lambda)?;
        let mut digits = String::new();
        digits.push(first);
        digits.push(last);
        sum += digits.parse::<i64>()?;
    }
    Ok(sum)
}

fn solve_inner<B: io::BufRead>(part: ProblemPart, br: B) -> Result<i64, AOCError> {
    match part {
        ProblemPart::P1 => part_one(br),
        ProblemPart::P2 => part_two(br),
    }
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    println!("{}", solve_inner(part, br)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    fn get_example_br() -> impl io::BufRead {
        io::BufReader::new(EXAMPLE.as_bytes())
    }

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn p1_example() {
        let result = solve_inner(ProblemPart::P1, get_example_br());
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn p2_example() {
        let result = solve_inner(ProblemPart::P2, io::BufReader::new(EXAMPLE2.as_bytes()));
        assert_eq!(result.unwrap(), 281);
    }
}
