use crate::{AOCError, ProblemPart};

use std::collections::{HashSet, VecDeque};
use std::io;
use std::num;
use std::str::FromStr;

#[derive(Debug)]
struct Move {
    steps: u64,
    dir: Direction,
}

struct Program {
    moves: Vec<Move>,
}

struct ProgramIterator<'a> {
    prog: &'a Program,
    idx: usize,
    states: VecDeque<Board>,
}

impl<'a> Iterator for ProgramIterator<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        match self.states.pop_front() {
            None => None,
            Some(board) => {
                if !self.states.is_empty() || self.idx >= self.prog.moves.len() {
                    return Some(board);
                }
                let next_m = &self.prog.moves[self.idx];
                self.idx += 1;
                let mut current_board = board.clone();
                for _ in 0..next_m.steps {
                    current_board = current_board.move_head(next_m.dir);
                    self.states.push_back(current_board.clone());
                }
                Some(board)
            }
        }
    }
}

impl Program {
    fn iter(&self, n: usize) -> ProgramIterator {
        let mut deq = VecDeque::new();
        deq.push_back(Board::new(n));
        ProgramIterator {
            prog: self,
            idx: 0,
            states: deq,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Board {
    knots: Vec<Point>,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct ParseMoveError {}

impl From<num::ParseIntError> for ParseMoveError {
    fn from(_err: num::ParseIntError) -> Self {
        ParseMoveError {}
    }
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comps: Vec<_> = s.split(' ').collect();
        if comps.len() != 2 {
            return Err(ParseMoveError {});
        }
        let dir = match comps[0] {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err(ParseMoveError {}),
        };
        let count = comps[1].parse::<u64>()?;
        Ok(Move {
            steps: count,
            dir: dir,
        })
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    fn apply_dir(mut self, dir: Direction) -> Point {
        match dir {
            Direction::U => self.y += 1,
            Direction::D => self.y -= 1,
            Direction::L => self.x -= 1,
            Direction::R => self.x += 1,
        };
        self
    }

    fn distance(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn different_row_and_column(&self, other: &Point) -> bool {
        self.x != other.x && self.y != other.y
    }

    fn within_range(&self, other: &Point) -> bool {
        let distance = self.distance(other);
        distance == 1 || distance == 2 && self.different_row_and_column(other)
    }
}

impl Board {
    // n is >= 2
    fn new(n: usize) -> Board {
        let mut knots = Vec::with_capacity(n);
        for _ in 0..n {
            knots.push(Point::new(0, 0));
        }
        Board { knots: knots }
    }

    // fn head_tail_distance(&self) -> u64 {
    //     self.head.distance(&self.tail)
    // }

    // fn head_tail_different_row_column(&self) -> bool {
    //     self.head.different_row_and_column(&self.tail)
    // }

    fn move_head(mut self, dir: Direction) -> Board {
        if self.knots.is_empty() {
            return self;
        }
        self.knots[0] = self.knots[0].apply_dir(dir);
        for i in 1..self.knots.len() {
            let leader = self.knots[i - 1];
            let follower = &mut self.knots[i];
            if leader.within_range(&follower) {
                continue;
            }
            // no longer an invariant, because of diagonal motion
            // assert!(leader.distance(follower) <= 3);
            let maybe_delta_y = if follower.y < leader.y {
                Some(Direction::U)
            } else if follower.y > leader.y {
                Some(Direction::D)
            } else {
                None
            };
            if let Some(delta_y) = maybe_delta_y {
                *follower = follower.apply_dir(delta_y);
            }
            let maybe_delta_x = if follower.x < leader.x {
                Some(Direction::R)
            } else if follower.x > leader.x {
                Some(Direction::L)
            } else {
                None
            };
            if let Some(delta_x) = maybe_delta_x {
                *follower = follower.apply_dir(delta_x);
            }
        }
        self
    }

    fn tail(&self) -> Point {
        assert!(!self.knots.is_empty());
        self.knots[self.knots.len() - 1]
    }
}

fn problem1(prog: Program) -> u64 {
    let mut tail_positions = HashSet::new();
    for board in prog.iter(2) {
        tail_positions.insert(board.tail());
    }
    tail_positions.len() as u64
}

fn problem2(prog: Program) -> u64 {
    let mut tail_positions = HashSet::new();
    for board in prog.iter(10) {
        tail_positions.insert(board.tail());
    }
    tail_positions.len() as u64
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let prog = Program {
        moves: br
            .lines()
            .map(|line| line.unwrap().parse::<Move>().unwrap())
            .collect(),
    };
    let result = match part {
        ProblemPart::P1 => problem1(prog),
        ProblemPart::P2 => problem2(prog),
    };
    println!("result: {}", result);
    Ok(())
}
