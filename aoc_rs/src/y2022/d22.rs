use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::io;
use std::ops::{Add, Mul, Sub};

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

fn compute_password(row_index: usize, col_index: usize, bearing: Bearing) -> u64 {
    let one_offset_row = (row_index + 1) as u64;
    let one_offset_col = (col_index + 1) as u64;
    1000 * one_offset_row + 4 * one_offset_col + bearing as u64
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
        compute_password(self.row, self.col, self.bearing)
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
    num_rows: usize,
    num_cols: usize,
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
        let num_rows = rows.len();
        Ok(Map {
            board: rows,
            num_rows: num_rows,
            num_cols: max_board_length,
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

type MapIndex = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }

    fn cross_prod(&self, rhs: &Point3D) -> Point3D {
        Point3D::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Point3D {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<Point3D> for usize {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Point3D {
        Point3D::new(
            self as i64 * rhs.x,
            self as i64 * rhs.y,
            self as i64 * rhs.z,
        )
    }
}

impl Mul<Point3D> for i64 {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Point3D {
        Point3D::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

fn isqrt(n: usize) -> Option<usize> {
    let f = (n as f64).sqrt();
    let upper = f.ceil() as usize;
    let lower = f.floor() as usize;
    if upper * upper == n {
        Some(upper)
    } else if lower * lower == n {
        Some(lower)
    } else {
        None
    }
}

struct Cube {
    map: Map,
    assoc_list: HashMap<Point3D, MapIndex>,
    side_length: usize,
}

impl Cube {
    fn get_unit_from_boundary(side_length: usize, point: Point3D) -> Point3D {
        let x_unit = if point.x == 0 {
            Some(Point3D::new(1, 0, 0))
        } else if point.x == (side_length + 1) as i64 {
            Some(Point3D::new(-1, 0, 0))
        } else {
            None
        };
        let y_unit = if point.y == 0 {
            Some(Point3D::new(0, 1, 0))
        } else if point.y == (side_length + 1) as i64 {
            Some(Point3D::new(0, -1, 0))
        } else {
            None
        };
        let z_unit = if point.z == 0 {
            Some(Point3D::new(0, 0, 1))
        } else if point.z == (side_length + 1) as i64 {
            Some(Point3D::new(0, 0, -1))
        } else {
            None
        };
        if (x_unit.is_some() as u8 + y_unit.is_some() as u8 + z_unit.is_some() as u8) != 1 {
            panic!(
                "cannot extract next dimension. side_length: {}, {:?}",
                side_length, point
            );
        }
        x_unit.or(y_unit).or(z_unit).unwrap()
    }

    fn add_face(
        map: &Map,
        side_length: usize,
        starting_2d_coord: MapIndex,
        starting_cube_coord: Point3D,
        col_delta: Point3D,
        row_delta: Point3D,
        accum: &mut HashMap<Point3D, MapIndex>,
        frontier: &mut Vec<(MapIndex, Point3D, Point3D, Point3D)>,
        explored: &mut HashSet<MapIndex>,
    ) {
        let (start_row, start_col) = starting_2d_coord;
        for (row_cnt, row) in (start_row..start_row + side_length).enumerate() {
            for (col_cnt, col) in (start_col..start_col + side_length).enumerate() {
                accum.insert(
                    starting_cube_coord + row_cnt * row_delta + col_cnt * col_delta,
                    (row, col),
                );
            }
        }
        // is there a face to the right?
        if start_col + side_length < map.num_cols
            && map.board[start_row][start_col + side_length] != Tile::Void
            && !explored.contains(&(start_row, start_col + side_length))
        {
            // can't add to col delta since col delta has either hit 0 or side_length.
            let new_col_delta = Cube::get_unit_from_boundary(side_length, starting_cube_coord);
            let new_start_cube_coord =
                starting_cube_coord + side_length * col_delta + new_col_delta;
            let new_board_idx = (start_row, start_col + side_length);
            explored.insert(new_board_idx);
            frontier.push((
                new_board_idx,
                new_start_cube_coord,
                new_col_delta,
                row_delta,
            ));
        }
        // is there a face to the left?
        if start_col > 0
            && map.board[start_row][start_col - 1] != Tile::Void
            && !explored.contains(&(start_row, start_col - side_length))
        {
            assert!(start_col >= side_length);
            let new_board_idx = (start_row, start_col - side_length);
            // because we're continuing to iterate with increasing column, we actually flip this around.
            let new_col_delta =
                (-1i64) * Cube::get_unit_from_boundary(side_length, starting_cube_coord);
            let new_start_cube_coord =
                starting_cube_coord - col_delta - (side_length) * new_col_delta;
            explored.insert(new_board_idx);
            frontier.push((
                new_board_idx,
                new_start_cube_coord,
                new_col_delta,
                row_delta,
            ));
        }
        // is there a face below?
        if start_row + side_length < map.num_rows
            && map.board[start_row + side_length][start_col] != Tile::Void
            && !explored.contains(&(start_row + side_length, start_col))
        {
            let new_row_delta = Cube::get_unit_from_boundary(side_length, starting_cube_coord);
            let new_start_cube_coord =
                starting_cube_coord + side_length * row_delta + new_row_delta;
            let new_board_idx = (start_row + side_length, start_col);
            explored.insert(new_board_idx);
            frontier.push((
                new_board_idx,
                new_start_cube_coord,
                col_delta,
                new_row_delta,
            ));
        }
    }

    fn fold_map(map: Map) -> Result<Cube, AOCError> {
        let side_length = Cube::get_side_length(&map)?;
        let mut frontier = Vec::new();
        let mut results = HashMap::new();
        let mut explored = HashSet::new();
        for col in 0..map.num_cols {
            if map.board[0][col] != Tile::Void {
                explored.insert((0, col));
                Cube::add_face(
                    &map,
                    side_length,
                    (0, col),
                    Point3D::new(1, 1, 0),
                    Point3D::new(1, 0, 0),
                    Point3D::new(0, 1, 0),
                    &mut results,
                    &mut frontier,
                    &mut explored,
                );

                break;
            }
        }
        while let Some((map_idx, starting_cube_pt, col_delta, row_delta)) = frontier.pop() {
            Cube::add_face(
                &map,
                side_length,
                map_idx,
                starting_cube_pt,
                col_delta,
                row_delta,
                &mut results,
                &mut frontier,
                &mut explored,
            );
        }
        if results.len() != side_length * side_length * 6 {
            return Err(aocerror!(
                "expected {} cube coordinates, only have {}",
                side_length * side_length * 6,
                results.len()
            ));
        }
        Ok(Cube {
            map: map,
            assoc_list: results,
            side_length: side_length,
        })
    }

    fn get_side_length(map: &Map) -> Result<usize, AOCError> {
        // count the number of tiles.
        // each face has side_length * side_length tiles.
        // 6 faces.
        // non-void tiles = 6 * s^2
        let num_non_voids = map
            .board
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&t| t != Tile::Void)
            .count();
        if num_non_voids % 6 != 0 {
            return Err(aocerror!(
                "cannot compute side length, number of non void tiles is: {}",
                num_non_voids
            ));
        }
        assert_eq!(num_non_voids % 6, 0);
        match isqrt(num_non_voids / 6) {
            Some(result) => Ok(result),
            None => Err(aocerror!(
                "cannot compute side length, number of non void tiles is: {}",
                num_non_voids
            )),
        }
    }

    fn cube_coord_to_map_index(&self, point: &Point3D) -> Option<MapIndex> {
        self.assoc_list.get(point).copied()
    }

    fn map_index_to_cube_coord(&self, map_index: &MapIndex) -> Option<Point3D> {
        self.assoc_list
            .iter()
            .find_map(|(point, mi)| if mi == map_index { Some(*point) } else { None })
    }

    fn initial_state(&self) -> CubeState {
        let initial_flat_state = State::initial_state(&self.map.board);
        // could maybe just hardcode Point3D::new(1, 1, 0)
        let initial_position = self
            .map_index_to_cube_coord(&(initial_flat_state.row, initial_flat_state.col))
            .unwrap();
        // this assumes the first face is an x-y face on the z=0 plane.
        CubeState {
            position: initial_position,
            heading: Point3D::new(1, 0, 0),
            normal: Point3D::new(0, 0, 1),
        }
    }

    fn apply_path_direction(&self, path_dir: PathDirection, state: &mut CubeState) {
        match path_dir {
            PathDirection::TurnLeft => state.heading = state.heading.cross_prod(&state.normal),
            PathDirection::TurnRight => state.heading = state.normal.cross_prod(&state.heading),
            PathDirection::Forward(n) => {
                for _ in 0..n {
                    if !self.step_forward(state) {
                        break;
                    }
                }
            }
        }
    }

    fn at_edge(&self, coord: &Point3D) -> bool {
        // an edge is where 2 planes meet.
        let on_plane = |val| (val == 0 || val == (self.side_length + 1) as i64) as u8;
        let planes_count = on_plane(coord.x) + on_plane(coord.y) + on_plane(coord.z);
        assert!(planes_count == 1 || planes_count == 2);
        planes_count == 2
    }

    fn step_forward(&self, state: &mut CubeState) -> bool {
        let mut new_coord = state.position + state.heading;
        let hit_edge = self.at_edge(&new_coord);
        if hit_edge {
            new_coord = new_coord + state.normal;
        }
        let map_index = match self.cube_coord_to_map_index(&new_coord) {
            Some(mi) => mi,
            None => {
                panic!("we broke: current state: {:?}, new coord: {:?}, map_index of original: {:?}, at edge: {}",
                       state, new_coord, self.cube_coord_to_map_index(&state.position), hit_edge
                );
            }
        };
        let tile = self.map.board[map_index.0][map_index.1];
        if tile == Tile::Wall {
            return false;
        }
        // we should never hit the void.
        assert!(tile != Tile::Void);
        state.position = new_coord;
        if hit_edge {
            let old_normal = state.normal;
            state.normal = -1i64 * state.heading;
            state.heading = old_normal;
        }
        true
    }

    fn bearing_from_coords(behind: &MapIndex, in_front: &MapIndex) -> Bearing {
        assert!(
            in_front.0 == behind.0 && in_front.1 != behind.1
                || in_front.0 != behind.0 && in_front.1 == behind.1
        );
        if in_front.0 == behind.0 {
            if in_front.1 > behind.1 {
                Bearing::Right
            } else {
                Bearing::Left
            }
        } else if in_front.0 > behind.0 {
            Bearing::Down
        } else {
            Bearing::Up
        }
    }

    fn password(&self, state: &CubeState) -> u64 {
        let map_index = self.cube_coord_to_map_index(&state.position).unwrap();
        let cube_coord_in_front = state.position + state.heading;
        let bearing =
            if let Some(map_index_in_front) = self.cube_coord_to_map_index(&cube_coord_in_front) {
                Cube::bearing_from_coords(&map_index, &map_index_in_front)
            } else {
                let cube_coord_behind = state.position - state.heading;
                // fails on a 1x1 face
                let behind = self.cube_coord_to_map_index(&cube_coord_behind).unwrap();
                Cube::bearing_from_coords(&behind, &map_index)
            };
        compute_password(map_index.0, map_index.1, bearing)
    }
}

#[derive(Debug, Clone)]
struct CubeState {
    position: Point3D,
    heading: Point3D,
    normal: Point3D,
}

fn problem1(map: Map) -> u64 {
    let mut state = State::initial_state(&map.board);
    for path_dir in map.path.iter() {
        state.apply_path_direction(*path_dir, &map.board);
    }
    state.password()
}

fn problem2(map: Map) -> u64 {
    let cube = Cube::fold_map(map).unwrap();
    let mut state = cube.initial_state();
    for path_dir in cube.map.path.iter() {
        cube.apply_path_direction(*path_dir, &mut state);
    }
    cube.password(&state)
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

    const EXAMPLE: [&'static str; 14] = [
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
    ];

    fn get_example_lines() -> Vec<String> {
        EXAMPLE.iter().map(|s| s.to_string()).collect::<Vec<_>>()
    }

    #[test]
    fn example() {
        let input = get_example_lines();
        let map = Map::from_lines(&input).unwrap();
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

    #[test]
    fn example_p2() {
        let input = get_example_lines();
        let result = problem2(Map::from_lines(&input).unwrap());
        assert_eq!(result, 5031);
    }

    #[test]
    fn cross_prod_on_initial_positions() {
        let initial_bearing = Point3D::new(1, 0, 0);
        let initial_normal = Point3D::new(0, 0, 1);
        assert_eq!(
            initial_normal.cross_prod(&initial_bearing),
            Point3D::new(0, 1, 0)
        );
        assert_eq!(
            initial_bearing.cross_prod(&initial_normal),
            Point3D::new(0, -1, 0)
        );
    }
}
