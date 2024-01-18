use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::io;

#[derive(Copy, Clone, Debug)]
enum Bearing {
    Right = 0,
    Down,
    Left,
    Up,
}

impl Bearing {
    fn turn_left(&self) -> Bearing {
        let val = *self as i64;
        Bearing::from_i64(val - 1)
    }

    fn turn_right(&self) -> Bearing {
        Bearing::from_i64(*self as i64 + 1)
    }

    fn from_i64(n: i64) -> Bearing {
        match n.rem_euclid(4) {
            0 => Bearing::Right,
            1 => Bearing::Down,
            2 => Bearing::Left,
            3 => Bearing::Up,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    row: usize,
    col: usize,
    bearing: Bearing,
}

fn incr_mod(n: usize, md: usize) -> usize {
    (n + 1) % md
}

// requires md > 0
fn decr_mod(n: usize, md: usize) -> usize {
    if n == 0 {
        md - 1
    } else {
        n - 1
    }
}

impl State {
    fn apply_path_direction(&mut self, dir: PathDirection, board: &Board) {
        match dir {
            PathDirection::TurnLeft => self.bearing = self.bearing.turn_left(),
            PathDirection::TurnRight => self.bearing = self.bearing.turn_right(),
            PathDirection::Forward(n) => {
                for _ in 0..n {
                    if !self.step_forward(board) {
                        break;
                    }
                }
            }
        }
    }

    fn advance(&mut self, board: &Board) {
        match self.bearing {
            Bearing::Right => self.col = incr_mod(self.col, board[self.row].len()),
            Bearing::Down => self.row = incr_mod(self.row, board.len()),
            Bearing::Left => self.col = decr_mod(self.col, board[self.row].len()),
            Bearing::Up => self.row = decr_mod(self.row, board.len()),
        }
    }

    fn tile_at(&self, board: &Board) -> Tile {
        board[self.row][self.col]
    }

    fn step_forward(&mut self, board: &Board) -> bool {
        let mut scout = self.clone();
        scout.advance(board);
        while scout.tile_at(board) == Tile::Void {
            scout.advance(board);
        }
        if scout.tile_at(board) == Tile::Open {
            *self = scout;
            true
        } else {
            false
        }
    }

    fn initial_state(board: &Board) -> State {
        let mut state = State {
            row: 0,
            col: 0,
            bearing: Bearing::Right,
        };
        while state.tile_at(board) == Tile::Void {
            state.advance(board);
        }
        state
    }

    fn password(&self) -> u64 {
        let one_offset_row = (self.row + 1) as u64;
        let one_offset_col = (self.col + 1) as u64;
        1000 * one_offset_row + 4 * one_offset_col + self.bearing as u64
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Void,
}

impl Tile {
    fn from_char(c: char) -> Result<Tile, AOCError> {
        match c {
            ' ' => Ok(Tile::Void),
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Open),
            _ => Err(aocerror!("cannot convert char to tile: {}", c)),
        }
    }
}

#[allow(dead_code)]
fn draw_world(state: &State, board: &Board) {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if state.row == row && state.col == col {
                let ch = match state.bearing {
                    Bearing::Right => '>',
                    Bearing::Down => 'v',
                    Bearing::Left => '<',
                    Bearing::Up => '^',
                };
                print!("{}", ch);
            } else {
                let ch = match board[row][col] {
                    Tile::Void => ' ',
                    Tile::Wall => '#',
                    Tile::Open => '.',
                };
                print!("{}", ch);
            }
        }
        println!("");
    }
}

type Board = Vec<Vec<Tile>>;

struct Map {
    board: Board,
    path: Vec<PathDirection>,
}

impl Map {
    fn from_lines(lines: &[String]) -> Result<Map, AOCError> {
        let mut max_board_length = 0;
        let mut empty_line_idx = 0;
        for (idx, line) in lines.iter().enumerate() {
            max_board_length = cmp::max(line.len(), max_board_length);
            if line.len() == 0 {
                empty_line_idx = idx;
                break;
            };
        }
        let (board_lines, path_lines) = lines.split_at(empty_line_idx);
        let mut rows = Vec::with_capacity(board_lines.len());
        for board_line in board_lines {
            let mut current_row = board_line
                .chars()
                .map(|c| Tile::from_char(c))
                .collect::<Result<Vec<_>, AOCError>>()?;
            while current_row.len() < max_board_length {
                current_row.push(Tile::Void);
            }
            rows.push(current_row);
        }
        if path_lines.len() < 2 {
            return Err(aocerror!(
                "Missing line containing path, suffix was: {:?}",
                path_lines
            ));
        }
        let path = PathDirection::parse_str(&path_lines[1])?;
        Ok(Map {
            board: rows,
            path: path,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum PathDirection {
    TurnLeft,
    TurnRight,
    Forward(u64),
}

impl PathDirection {
    fn parse_str(s: &str) -> Result<Vec<PathDirection>, AOCError> {
        let mut result = Vec::with_capacity(s.len());
        let mut parsed_s = s;
        while let Some(first_char) = parsed_s.chars().next() {
            if first_char.is_digit(10) {
                match parsed_s.find(&['L', 'R']) {
                    None => {
                        result.push(PathDirection::Forward(parsed_s.parse::<u64>()?));
                        break;
                    }
                    Some(end_num_idx) => {
                        result.push(PathDirection::Forward(
                            parsed_s[..end_num_idx].parse::<u64>()?,
                        ));
                        parsed_s = &parsed_s[end_num_idx..];
                    }
                }
            } else {
                let entry = match first_char {
                    'L' => PathDirection::TurnLeft,
                    'R' => PathDirection::TurnRight,
                    _ => {
                        return Err(aocerror!(
                            "cannot parse PathDirection from char: {}",
                            first_char
                        ))
                    }
                };
                result.push(entry);
                if parsed_s.len() > 1 {
                    parsed_s = &parsed_s[1..];
                } else {
                    break;
                }
            }
        }
        Ok(result)
    }
}

fn problem1(map: Map) -> u64 {
    let mut state = State::initial_state(&map.board);
    for path_dir in map.path.iter() {
        state.apply_path_direction(*path_dir, &map.board);
    }
    state.password()
}

fn problem2(_: Map) -> u64 {
    panic!();
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let map = Map::from_lines(&br.lines().collect::<Result<Vec<_>, _>>()?)?;
    let result = match part {
        ProblemPart::P1 => problem1(map),
        ProblemPart::P2 => problem2(map),
    };
    println!("result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tiny_path() {
        let path = "10R5";
        assert_eq!(
            PathDirection::parse_str(path).unwrap(),
            vec![
                PathDirection::Forward(10),
                PathDirection::TurnRight,
                PathDirection::Forward(5),
            ]
        );
    }

    #[test]
    fn parse_tiny_path2() {
        let path = "R52L46R";
        assert_eq!(
            PathDirection::parse_str(path).unwrap(),
            vec![
                PathDirection::TurnRight,
                PathDirection::Forward(52),
                PathDirection::TurnLeft,
                PathDirection::Forward(46),
                PathDirection::TurnRight
            ]
        );
    }

    #[test]
    fn example() {
        let v = vec![
            "        ...#",
            "        .#..",
            "        #...",
            "        ....",
            "...#.......#",
            "........#...",
            "..#....#....",
            "..........#.",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let map = Map::from_lines(&v).unwrap();
        let mut state = State::initial_state(&map.board);
        for path_dir in map.path.iter() {
            let original_state = state.clone();
            state.apply_path_direction(*path_dir, &map.board);
            if original_state.row != state.row || original_state.col != state.col {
                draw_world(&original_state, &map.board);
                println!("");
            }
        }
        draw_world(&state, &map.board);
        assert_eq!(state.password(), 6032);
    }
}
