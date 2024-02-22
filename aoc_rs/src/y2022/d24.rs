use crate::{aocerror, AOCError, ProblemPart};

use std::cmp::{min, Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::io;

#[derive(Debug, Copy, Clone)]
// up:    0001
// down:  0010
// left:  0100
// right: 1000
struct Blizzards {
    mask: u8,
}

impl Blizzards {
    fn make_empty() -> Blizzards {
        Blizzards { mask: 0 }
    }

    fn make_up() -> Blizzards {
        Blizzards { mask: 1 }
    }

    fn set_up(&mut self) {
        self.mask |= 1;
    }

    fn has_up(&self) -> bool {
        self.mask & 1 > 0
    }

    fn make_down() -> Blizzards {
        Blizzards { mask: 1 << 1 }
    }

    fn set_down(&mut self) {
        self.mask |= 1 << 1;
    }

    fn has_down(&self) -> bool {
        self.mask & (1 << 1) > 0
    }

    fn make_left() -> Blizzards {
        Blizzards { mask: 1 << 2 }
    }

    fn set_left(&mut self) {
        self.mask |= 1 << 2;
    }

    fn has_left(&self) -> bool {
        self.mask & (1 << 2) > 0
    }

    fn make_right() -> Blizzards {
        Blizzards { mask: 1 << 3 }
    }

    fn set_right(&mut self) {
        self.mask |= 1 << 3;
    }

    fn has_right(&self) -> bool {
        self.mask & (1 << 3) > 0
    }

    fn from_char(c: char) -> Result<Blizzards, AOCError> {
        match c {
            '^' => Ok(Blizzards::make_up()),
            '<' => Ok(Blizzards::make_left()),
            '>' => Ok(Blizzards::make_right()),
            'v' => Ok(Blizzards::make_down()),
            '.' => Ok(Blizzards::make_empty()),
            _ => Err(aocerror!("unrecognized character: {}", c)),
        }
    }

    fn has_blizzard(&self) -> bool {
        self.mask > 0
    }
}

struct Valley {
    entries: Vec<Vec<Blizzards>>,
}

fn mod_decr(a: usize, modulus: usize) -> usize {
    if a == 0 {
        modulus - 1
    } else {
        a - 1
    }
}

fn mod_incr(a: usize, modulus: usize) -> usize {
    if a == modulus - 1 {
        0
    } else {
        a + 1
    }
}

impl Valley {
    fn blizzard_at_position(&self, pos: Position) -> bool {
        match pos {
            Position::Start => false,
            Position::End => false,
            Position::Coords(row, col) => self.entries[row][col].has_blizzard(),
        }
    }

    fn advance(&self) -> Valley {
        let n_rows = self.entries.len();
        let n_cols = self.entries[0].len();
        let mut result = Vec::with_capacity(n_rows);
        for _ in 0..n_rows {
            let mut row = Vec::with_capacity(n_cols);
            for _ in 0..n_cols {
                row.push(Blizzards::make_empty());
            }
            result.push(row);
        }
        for row in 0..n_rows {
            for col in 0..n_cols {
                let entry = self.entries[row][col];
                if entry.has_up() {
                    result[mod_decr(row, n_rows)][col].set_up();
                }
                if entry.has_down() {
                    result[mod_incr(row, n_rows)][col].set_down();
                }
                if entry.has_left() {
                    result[row][mod_decr(col, n_cols)].set_left();
                }
                if entry.has_right() {
                    result[row][mod_incr(col, n_cols)].set_right();
                }
            }
        }
        Valley { entries: result }
    }

    fn parse<B: io::BufRead>(br: B) -> Result<Valley, AOCError> {
        let mut rows = Vec::new();
        for maybe_line in br.lines() {
            let line = maybe_line?;
            let second_is_wall = line.chars().nth(1) == Some('#');
            let third_is_wall = line.chars().nth(2) == Some('#');
            if !second_is_wall && third_is_wall {
                // first line
            } else if !second_is_wall && !third_is_wall {
                let mut new_row = Vec::new();
                // body line
                for ch in line.chars() {
                    match ch {
                        '#' => (),
                        _ => new_row.push(Blizzards::from_char(ch)?),
                    }
                }
                rows.push(new_row);
            } else if second_is_wall && third_is_wall {
                // last line
            } else {
                panic!("unreachable");
            }
        }
        Ok(Valley { entries: rows })
    }
}

struct ValleyCache {
    valleys: Vec<Valley>,
}

impl ValleyCache {
    fn new(valley: Valley) -> ValleyCache {
        ValleyCache {
            valleys: vec![valley],
        }
    }

    fn get(&mut self, idx: usize) -> &Valley {
        if idx < self.valleys.len() {
            return &self.valleys[idx];
        }
        assert!(idx == self.valleys.len());
        assert!(self.valleys.len() > 0);
        let new_valley = self.valleys[self.valleys.len() - 1].advance();
        self.valleys.push(new_valley);
        &self.valleys[self.valleys.len() - 1]
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
enum Position {
    Start,
    End,
    Coords(usize, usize),
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    pos: Position,
    score: usize,
    time: usize,
}

fn ne_ord_or<O: Ord>(ord: Ordering, a: &O, b: &O) -> Ordering {
    if ord != Ordering::Equal {
        ord
    } else {
        a.cmp(b)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        ne_ord_or(
            ne_ord_or(self.score.cmp(&other.score), &self.time, &other.time),
            &self.pos,
            &other.pos,
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait Frontier {
    fn n_rows(&self) -> usize;
    fn n_cols(&self) -> usize;
    fn add_state(&mut self, pos: Position, time: usize) -> ();
    fn pop(&mut self) -> Option<State>;
    fn is_terminal(&self, state: &State) -> bool;
}

#[derive(Debug)]
struct AStarBackwards {
    heap: BinaryHeap<State>,
    n_rows: usize,
    n_cols: usize,
}

impl Frontier for AStarBackwards {
    fn add_state(&mut self, pos: Position, time: usize) {
        let state = State {
            pos: pos,
            score: self.score_position(&pos),
            time: time,
        };
        self.heap.push(state)
    }

    fn pop(&mut self) -> Option<State> {
        match self.heap.pop() {
            Some(state) => Some(state),
            None => None,
        }
    }

    fn n_rows(&self) -> usize {
        self.n_rows
    }

    fn n_cols(&self) -> usize {
        self.n_cols
    }

    fn is_terminal(&self, state: &State) -> bool {
        state.pos == Position::Start
    }
}

impl AStarBackwards {
    fn new(n_rows: usize, n_cols: usize, start_time: usize) -> AStarBackwards {
        let mut frontier = AStarBackwards {
            heap: BinaryHeap::new(),
            n_rows: n_rows,
            n_cols: n_cols,
        };
        frontier.add_state(Position::End, start_time);
        frontier
    }

    fn score_position(&self, pos: &Position) -> usize {
        match pos {
            Position::Start => self.n_rows + 1 + self.n_cols - 1,
            Position::End => 0,
            Position::Coords(row, col) => {
                assert!(&self.n_rows >= row);
                assert!(self.n_cols >= col + 1);
                self.n_rows - row + self.n_cols - 1 - col
            }
        }
    }
}

#[derive(Debug)]
struct AStarFrontier {
    heap: BinaryHeap<Reverse<State>>,
    n_rows: usize,
    n_cols: usize,
}

impl Frontier for AStarFrontier {
    fn add_state(&mut self, pos: Position, time: usize) {
        self.heap.push(Reverse(State {
            pos: pos,
            score: self.score_position(&pos),
            time: time,
        }))
    }

    fn pop(&mut self) -> Option<State> {
        match self.heap.pop() {
            Some(Reverse(state)) => Some(state),
            None => None,
        }
    }

    fn n_rows(&self) -> usize {
        self.n_rows
    }

    fn n_cols(&self) -> usize {
        self.n_cols
    }

    fn is_terminal(&self, state: &State) -> bool {
        state.pos == Position::End
    }
}

impl AStarFrontier {
    fn new(n_rows: usize, n_cols: usize, start_time: usize) -> AStarFrontier {
        let mut frontier = AStarFrontier {
            heap: BinaryHeap::new(),
            n_rows: n_rows,
            n_cols: n_cols,
        };
        frontier.add_state(Position::Start, start_time);
        frontier
    }

    fn score_position(&self, pos: &Position) -> usize {
        match pos {
            Position::Start => self.n_rows + 1 + self.n_cols - 1,
            Position::End => 0,
            Position::Coords(row, col) => {
                assert!(&self.n_rows >= row);
                assert!(self.n_cols >= col + 1);
                self.n_rows - row + self.n_cols - 1 - col
            }
        }
    }
}

fn maybe_add_state<F: Frontier>(
    pos: Position,
    time: usize,
    frontier: &mut F,
    explored: &mut HashSet<(Position, usize)>,
    valley: &Valley,
) {
    if !valley.blizzard_at_position(pos) && !explored.contains(&(pos, time)) {
        frontier.add_state(pos, time);
        explored.insert((pos, time));
    }
}

fn least_minutes_generic<F: Frontier>(mut frontier: F, valley_cache: &mut ValleyCache) -> u64 {
    let mut shortest_path_length = usize::MAX;
    let mut explored = HashSet::new();
    while let Some(state) = frontier.pop() {
        if state.time >= shortest_path_length {
            continue;
        }
        if frontier.is_terminal(&state) {
            shortest_path_length = min(shortest_path_length, state.time);
            continue;
        }
        // iterate my neighbours.
        let new_time = state.time + 1;
        let new_valley = valley_cache.get(new_time);
        match state.pos {
            Position::End => {
                let possible_move = Position::Coords(frontier.n_rows() - 1, frontier.n_cols() - 1);
                maybe_add_state(
                    possible_move,
                    new_time,
                    &mut frontier,
                    &mut explored,
                    &new_valley,
                );
                maybe_add_state(
                    Position::End,
                    new_time,
                    &mut frontier,
                    &mut explored,
                    &new_valley,
                );
            }
            Position::Start => {
                let first_pos = Position::Coords(0, 0);
                maybe_add_state(
                    first_pos,
                    new_time,
                    &mut frontier,
                    &mut explored,
                    &new_valley,
                );
                maybe_add_state(
                    Position::Start,
                    new_time,
                    &mut frontier,
                    &mut explored,
                    &new_valley,
                );
            }
            Position::Coords(row, col) => {
                if row == frontier.n_rows() - 1 && col == frontier.n_cols() - 1 {
                    maybe_add_state(
                        Position::End,
                        new_time,
                        &mut frontier,
                        &mut explored,
                        &new_valley,
                    );
                }
                if row == 0 && col == 0 {
                    let new_pos = Position::Start;
                    maybe_add_state(new_pos, new_time, &mut frontier, &mut explored, &new_valley);
                }
                if row > 0 {
                    let above = Position::Coords(row - 1, col);
                    maybe_add_state(above, new_time, &mut frontier, &mut explored, &new_valley);
                }
                if col > 0 {
                    let left = Position::Coords(row, col - 1);
                    maybe_add_state(left, new_time, &mut frontier, &mut explored, &new_valley);
                }
                if row < frontier.n_rows() - 1 {
                    let below = Position::Coords(row + 1, col);
                    maybe_add_state(below, new_time, &mut frontier, &mut explored, &new_valley);
                }
                if col < frontier.n_cols() - 1 {
                    let right = Position::Coords(row, col + 1);
                    maybe_add_state(right, new_time, &mut frontier, &mut explored, &new_valley);
                }
                // waiting in place.
                maybe_add_state(
                    state.pos,
                    new_time,
                    &mut frontier,
                    &mut explored,
                    &new_valley,
                );
            }
        }
    }
    shortest_path_length as u64
}

fn least_minutes(initial: Valley) -> u64 {
    let n_rows = initial.entries.len();
    let n_cols = initial.entries[0].len();
    let frontier = AStarFrontier::new(n_rows, n_cols, 0);
    let mut valley_cache = ValleyCache::new(initial);
    least_minutes_generic(frontier, &mut valley_cache)
}

fn there_and_back_again(initial: Valley) -> u64 {
    let n_rows = initial.entries.len();
    let n_cols = initial.entries[0].len();
    let frontier = AStarFrontier::new(n_rows, n_cols, 0);
    let mut valley_cache = ValleyCache::new(initial);
    let first_trip_time = least_minutes_generic(frontier, &mut valley_cache);
    let second_trip_time = least_minutes_generic(
        AStarBackwards::new(n_rows, n_cols, first_trip_time as usize),
        &mut valley_cache,
    );
    let third_trip_time = least_minutes_generic(
        AStarFrontier::new(n_rows, n_cols, second_trip_time as usize),
        &mut valley_cache,
    );
    third_trip_time
}

fn solve_inner<B: io::BufRead>(part: ProblemPart, br: B) -> Result<u64, AOCError> {
    let valley = Valley::parse(br)?;
    match part {
        ProblemPart::P1 => Ok(least_minutes(valley)),
        ProblemPart::P2 => Ok(there_and_back_again(valley)),
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

    const EXAMPLE: &str = "#.######\
       \n#>>.<^<#\
       \n#.<..<<#\
       \n#>v.><>#\
       \n#<^v^^>#\
       \n######.#";

    fn get_example_br() -> impl io::BufRead {
        io::BufReader::new(EXAMPLE.as_bytes())
    }

    #[test]
    fn example_p1() {
        let valley = Valley::parse(get_example_br()).unwrap();
        let actual = least_minutes(valley);
        assert_eq!(actual, 18);
    }

    #[test]
    fn example_p2() {
        let valley = Valley::parse(get_example_br()).unwrap();
        let actual = there_and_back_again(valley);
        assert_eq!(actual, 54);
    }
}
