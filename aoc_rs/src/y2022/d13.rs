use crate::{AOCError, ProblemPart};

use std::cmp::{Ord, Ordering};
use std::io;
use std::iter;
use std::str;

#[derive(Debug, PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Number(u64),
    Comma,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum NumberList {
    Number(u64),
    List(Vec<NumberList>),
}

impl NumberList {
    fn wrap_number_in_list(n: u64) -> NumberList {
        NumberList::from(vec![NumberList::from(n)])
    }

    fn compare_lists(l: &[NumberList], r: &[NumberList]) -> Ordering {
        for (l_item, r_item) in l.iter().zip(r.iter()) {
            let item_cmp = l_item.cmp(r_item);
            if item_cmp == Ordering::Equal {
                continue;
            }
            return item_cmp;
        }
        l.len().cmp(&r.len())
    }

    fn wrap_in_list(self) -> NumberList {
        NumberList::from(vec![self])
    }
}

impl From<u64> for NumberList {
    fn from(n: u64) -> Self {
        NumberList::Number(n)
    }
}

impl From<Vec<NumberList>> for NumberList {
    fn from(l: Vec<NumberList>) -> Self {
        NumberList::List(l)
    }
}

impl Ord for NumberList {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (NumberList::Number(l), NumberList::Number(r)) => l.cmp(r),
            (NumberList::Number(l), NumberList::List(_)) => {
                NumberList::wrap_number_in_list(*l).cmp(other)
            }
            (NumberList::List(_), NumberList::Number(r)) => {
                self.cmp(&NumberList::wrap_number_in_list(*r))
            }
            (NumberList::List(l), NumberList::List(r)) => NumberList::compare_lists(l, r),
        }
    }
}

impl PartialOrd for NumberList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list_items<'a, I>(tokens: &mut iter::Peekable<I>) -> NumberList
where
    I: Iterator<Item = &'a Token>,
{
    let mut list_items = Vec::new();
    while let Some(list_token) = tokens.peek() {
        match list_token {
            Token::CloseBracket => {
                tokens.next();
                return NumberList::List(list_items);
            }
            _ => list_items.push(parse_helper(tokens).unwrap()),
        }
    }
    // got to the end of the tokens without matching closing delimiter
    panic!();
}

fn parse_helper<'a, I>(tokens: &mut iter::Peekable<I>) -> Option<NumberList>
where
    I: Iterator<Item = &'a Token>,
{
    while let Some(token) = tokens.next() {
        match token {
            Token::CloseBracket => panic!(),
            Token::Comma => continue,
            Token::Number(n) => return Some(NumberList::Number(*n)),
            Token::OpenBracket => return Some(parse_list_items(tokens)),
        }
    }
    None
}

fn parse(tokens: &[Token]) -> Option<NumberList> {
    parse_helper(&mut tokens.iter().peekable())
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn tokenize_helper(s: &str) -> (Option<Token>, &str) {
    if let Some(ch) = s.chars().next() {
        match ch {
            '[' => (Some(Token::OpenBracket), &s[1..]),
            ']' => (Some(Token::CloseBracket), &s[1..]),
            ',' => (Some(Token::Comma), &s[1..]),
            d if is_digit(d) => {
                let next_non_digit_idx = s.find(|c| !is_digit(c)).unwrap_or(s.len());
                let num = s[0..next_non_digit_idx].parse::<u64>().unwrap();
                (Some(Token::Number(num)), &s[next_non_digit_idx..])
            }
            _ => panic!(),
        }
    } else {
        (None, s)
    }
}

fn tokenize(mut s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    while let (Some(t), rest_s) = tokenize_helper(s) {
        tokens.push(t);
        s = rest_s;
    }
    tokens
}

fn parse_problem(lines: &[String]) -> Vec<(NumberList, NumberList)> {
    let mut idx = 0;
    let mut pairs = Vec::new();
    while idx < lines.len() {
        let p1 = parse(&tokenize(&lines[idx])).unwrap();
        let p2 = parse(&tokenize(&lines[idx + 1])).unwrap();
        pairs.push((p1, p2));
        assert!(idx + 2 >= lines.len() || lines[idx + 2].is_empty());
        idx += 3;
    }
    pairs
}

fn problem1(pairs: Vec<(NumberList, NumberList)>) -> usize {
    let mut sum = 0;
    for (raw_idx, (lhs, rhs)) in pairs.iter().enumerate() {
        let idx = raw_idx + 1;
        if lhs < rhs {
            sum += idx;
        }
    }
    sum
}

fn problem2(pairs: Vec<(NumberList, NumberList)>) -> usize {
    let mut all_packets = Vec::with_capacity(pairs.len() * 2);
    for (lhs, rhs) in pairs.into_iter() {
        all_packets.push(lhs);
        all_packets.push(rhs);
    }
    let divider_first = NumberList::from(2).wrap_in_list().wrap_in_list();
    let divider_second = NumberList::from(6).wrap_in_list().wrap_in_list();
    all_packets.push(divider_first.clone());
    all_packets.push(divider_second.clone());
    all_packets.sort();
    let first_idx = all_packets.iter().position(|x| x == &divider_first);
    let second_idx = all_packets.iter().position(|x| x == &divider_second);
    (first_idx.unwrap() + 1) * (second_idx.unwrap() + 1)
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let lines: Vec<_> = br.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let problem = parse_problem(&lines);
    let result = match part {
        ProblemPart::P1 => problem1(problem),
        ProblemPart::P2 => problem2(problem),
    };
    println!("result: {}", result);
    Ok(())
}
