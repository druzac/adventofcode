use crate::{AOCError, ProblemPart};

use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::io;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn hamming_distance(&self, other: &Coord) -> u32 {
        let mut result = 0;
        if self.row != other.row {
            result += 1;
        }
        if self.col != other.col {
            result += 1;
        }
        result
    }

    fn new(i: usize, j: usize) -> Coord {
        Coord { row: i, col: j }
    }
}

struct Grid {
    heights: Vec<Vec<u32>>,
    start: Coord,
    end: Coord,
}

fn char_to_height(c: char) -> u32 {
    match c {
        'S' => char_to_height('a'),
        'E' => char_to_height('z'),
        x => x as u32 - 'a' as u32,
    }
}

impl Grid {
    fn height(&self, point: Coord) -> u32 {
        self.heights[point.row][point.col]
    }

    fn neighbours(&self, point: Coord) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        let start_row = if point.row > 0 { point.row - 1 } else { 0 };
        let start_col = if point.col > 0 { point.col - 1 } else { 0 };
        let max_reachable_height = self.height(point) + 1;
        for i in start_row..cmp::min(point.row + 2, self.heights.len()) {
            for j in start_col..cmp::min(point.col + 2, self.heights[i].len()) {
                let cand_neighbour = Coord { row: i, col: j };
                if point.hamming_distance(&cand_neighbour) == 1
                    && self.height(cand_neighbour) <= max_reachable_height
                {
                    neighbours.push(cand_neighbour);
                }
            }
        }
        neighbours
    }
}

fn parse(lines: Vec<String>) -> Grid {
    let mut heights: Vec<Vec<u32>> = Vec::with_capacity(lines.len());
    let mut start = Coord { row: 0, col: 0 };
    let mut end = Coord { row: 0, col: 0 };
    for (i, line) in lines.iter().enumerate() {
        let mut row: Vec<u32> = Vec::with_capacity(line.len());
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start.row = i;
                start.col = j;
            }
            if ch == 'E' {
                end.row = i;
                end.col = j;
            }
            // println!("i: {}, j: {}, char: {}, height: {}",
            //          i, j, ch, char_to_height(ch));
            row.push(char_to_height(ch));
        }
        heights.push(row);
    }
    Grid {
        heights: heights,
        start: start,
        end: end,
    }
}

fn problem1(grid: &Grid) -> Option<u64> {
    let mut frontier: VecDeque<(Coord, u64)> = VecDeque::new();
    frontier.push_back((grid.start, 0));
    let mut explored: HashSet<Coord> = HashSet::new();
    explored.insert(grid.start);
    while let Some((current_coord, path_length)) = frontier.pop_front() {
        for neighbour in grid.neighbours(current_coord) {
            if neighbour == grid.end {
                return Some(path_length + 1);
            }
            if explored.contains(&neighbour) {
                continue;
            }
            frontier.push_back((neighbour, path_length + 1));
            explored.insert(neighbour);
        }
    }
    None
}

// oooor... start from end, and go backwards. yep, that'd be better.
// definitely more efficient.
fn problem2(mut grid: Grid) -> u64 {
    let mut lowest = u64::MAX;
    for i in 0..grid.heights.len() {
        for j in 0..grid.heights[i].len() {
            let cand = Coord::new(i, j);
            if grid.height(cand) == 0 {
                grid.start = cand;
                lowest = cmp::min(lowest, problem1(&grid).unwrap_or(u64::MAX));
            }
        }
    }
    lowest
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let lines: Vec<_> = br.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let graph = parse(lines);
    let result = match part {
        ProblemPart::P1 => problem1(&graph).unwrap(),
        ProblemPart::P2 => problem2(graph),
    };
    println!("result: {}", result);
    Ok(())
}
