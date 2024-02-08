use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::fmt;
use std::io;
use std::ops::Sub;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn new(x: i64, y: i64) -> Point2D {
        Point2D { x: x, y: y }
    }

    fn pointwise_max(&self, rhs: &Point2D) -> Point2D {
        Point2D::new(cmp::max(self.x, rhs.x), cmp::max(self.y, rhs.y))
    }

    fn pointwise_min(&self, rhs: &Point2D) -> Point2D {
        Point2D::new(cmp::min(self.x, rhs.x), cmp::min(self.y, rhs.y))
    }

    fn rectangle_area(&self, other: &Point2D) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl Sub<&Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: &Point2D) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, point: &Point2D) -> Point2D {
        match self {
            Direction::North => Point2D::new(point.x, point.y - 1),
            Direction::South => Point2D::new(point.x, point.y + 1),
            Direction::West => Point2D::new(point.x - 1, point.y),
            Direction::East => Point2D::new(point.x + 1, point.y),
        }
    }

    fn all_bearings<'a, 'b>(&'a self, point: &'b Point2D) -> AllBearingsIterator<'b> {
        AllBearingsIterator {
            cnt: 0,
            dir: *self,
            original_point: point,
        }
    }
}

struct AllBearingsIterator<'a> {
    cnt: u8,
    dir: Direction,
    original_point: &'a Point2D,
}

impl<'a> Iterator for AllBearingsIterator<'a> {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt == 3 {
            return None;
        }
        let next = match self.dir {
            Direction::North => match self.cnt {
                0 => Direction::North.apply(self.original_point),
                1 => Direction::West.apply(&Direction::North.apply(self.original_point)),
                2 => Direction::East.apply(&Direction::North.apply(self.original_point)),
                _ => panic!(),
            },
            Direction::South => match self.cnt {
                0 => Direction::South.apply(self.original_point),
                1 => Direction::West.apply(&Direction::South.apply(self.original_point)),
                2 => Direction::East.apply(&Direction::South.apply(self.original_point)),
                _ => panic!(),
            },
            Direction::West => match self.cnt {
                0 => Direction::West.apply(self.original_point),
                1 => Direction::North.apply(&Direction::West.apply(self.original_point)),
                2 => Direction::South.apply(&Direction::West.apply(self.original_point)),
                _ => panic!(),
            },
            Direction::East => match self.cnt {
                0 => Direction::East.apply(self.original_point),
                1 => Direction::North.apply(&Direction::East.apply(self.original_point)),
                2 => Direction::South.apply(&Direction::East.apply(self.original_point)),
                _ => panic!(),
            },
        };
        self.cnt += 1;
        Some(next)
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Direction {
        match value % 4 {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!("unreachable"),
        }
    }
}

struct ElfDirectionIterator {
    idx: u8,
    cnt: u8,
}

impl Iterator for ElfDirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt == 4 {
            return None;
        }
        let old_idx = self.idx;
        self.idx += 1;
        self.cnt += 1;
        Some(Direction::from(old_idx))
    }
}

#[derive(Debug)]
struct Field {
    map: HashSet<Point2D>,
    direction_idx: u8,
}

impl Field {
    fn parse<B: io::BufRead>(br: B) -> Result<Field, AOCError> {
        let mut map = HashSet::new();
        for (row, maybe_line) in br.lines().enumerate() {
            for (col, ch) in maybe_line?.chars().enumerate() {
                match ch {
                    '#' => {
                        map.insert(Point2D::new(col as i64, row as i64));
                        ()
                    }
                    '.' => (),
                    p => return Err(aocerror!("unrecognized character: {}", p)),
                }
            }
        }
        Ok(Field {
            map: map,
            direction_idx: 0,
        })
    }

    fn has_neighbour(&self, dir: Direction, point: &Point2D) -> bool {
        dir.all_bearings(point)
            .find(|point| {
                let result = self.map.contains(&point);
                result
            })
            .is_some()
    }

    fn directions(&self) -> ElfDirectionIterator {
        ElfDirectionIterator {
            idx: self.direction_idx,
            cnt: 0,
        }
    }

    // if elf has no neighbours, returns None - leave Elf.idx alone.
    // if elf has neighbours and there is a move possible, return Some(new_point) - increment Elf.idx.
    // if elf has neighbours and there is no move possible, return None - increment Elf.idx.
    fn elf_proposal(
        &self,
        coord: &Point2D,
        proposals: &mut HashMap<Point2D, Point2D>,
        impossible_moves: &mut HashSet<Point2D>,
    ) {
        let mut has_any_neighbour = false;
        let mut proposed_move = None;
        for dir in self.directions() {
            let neighbour_here = self.has_neighbour(dir, coord);
            has_any_neighbour |= neighbour_here;
            if !neighbour_here && proposed_move.is_none() {
                proposed_move = Some(dir.apply(coord));
            }
        }
        if !has_any_neighbour {
            return;
        }
        match proposed_move {
            Some(new_coord) => {
                if proposals.contains_key(&new_coord) {
                    proposals.remove(&new_coord).unwrap();
                    impossible_moves.insert(new_coord);
                } else if !impossible_moves.contains(&new_coord) {
                    proposals.insert(new_coord, coord.clone());
                }
            }
            _ => (),
        }
    }

    fn execute_round(&mut self) -> bool {
        let mut proposed_moves = HashMap::new();
        let mut impossible_moves = HashSet::new();
        for coord in self.map.iter() {
            self.elf_proposal(coord, &mut proposed_moves, &mut impossible_moves);
        }
        let made_change = !proposed_moves.is_empty();
        for (new_coord, old_coord) in proposed_moves.drain() {
            match self.map.remove(&old_coord) {
                true => {
                    self.map.insert(new_coord);
                    ()
                }
                false => panic!("unreachable"),
            }
        }
        self.direction_idx = (self.direction_idx + 1) % 4;
        made_change
    }

    fn count_empty_ground_tiles(&self) -> u64 {
        let mut max_point = Point2D::new(i64::MIN, i64::MIN);
        let mut min_point = Point2D::new(i64::MAX, i64::MAX);
        for coord in self.map.iter() {
            max_point = max_point.pointwise_max(coord);
            min_point = min_point.pointwise_min(coord);
        }
        max_point.rectangle_area(&min_point) - (self.map.len() as u64)
    }

    #[allow(dead_code)]
    fn normalize(&mut self) {
        let mut min_point = Point2D::new(i64::MAX, i64::MAX);
        for coord in self.map.iter() {
            min_point = min_point.pointwise_min(coord);
        }
        self.map = self
            .map
            .drain()
            .map(|coord| coord - &min_point)
            .collect::<HashSet<_>>()
    }

    #[allow(dead_code)]
    fn elf_points<'a>(&'a self) -> impl Iterator<Item = &'a Point2D> + 'a {
        self.map.iter()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut min_point = Point2D::new(i64::MAX, i64::MAX);
        let mut max_point = Point2D::new(i64::MIN, i64::MIN);
        for coord in self.map.iter() {
            min_point = min_point.pointwise_min(coord);
            max_point = max_point.pointwise_max(coord);
        }
        for y in min_point.y..(max_point.y + 1) {
            for x in min_point.x..(max_point.x + 1) {
                let c = if self.map.contains(&Point2D::new(x, y)) {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn problem1(mut field: Field) -> u64 {
    for _ in 0..10 {
        field.execute_round();
    }
    field.count_empty_ground_tiles()
}

fn problem2(mut field: Field) -> u64 {
    let mut cnt = 0;
    loop {
        cnt += 1;
        if !field.execute_round() {
            break;
        }
    }
    cnt
}

fn solve_inner<B: io::BufRead>(part: ProblemPart, br: B) -> Result<u64, AOCError> {
    let field = Field::parse(br)?;
    match part {
        ProblemPart::P1 => Ok(problem1(field)),
        ProblemPart::P2 => Ok(problem2(field)),
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
    use std::io::prelude::*;

    const EXAMPLE: &str = "\
\n..............\
\n..............\
\n.......#......\
\n.....###.#....\
\n...#...#.#....\
\n....#...##....\
\n...#.###......\
\n...##.#.##....\
\n....#..#......\
\n..............\
\n..............\
\n..............";

    fn get_example_br() -> impl BufRead {
        io::BufReader::new(EXAMPLE.as_bytes())
    }

    fn dump_normalized_coords(field: &mut Field) -> HashSet<&Point2D> {
        field.normalize();
        field.elf_points().collect::<HashSet<_>>()
    }

    fn compare_for_n_rounds(
        n: u64,
        start: &str,
        final_state: &str,
    ) -> Result<(HashSet<Point2D>, HashSet<Point2D>), AOCError> {
        let mut field = Field::parse(io::BufReader::new(start.as_bytes()))?;
        let mut expected = Field::parse(io::BufReader::new(final_state.as_bytes()))?;
        for _ in 0..n {
            field.execute_round();
        }
        println!("Actual field is:\n{}", field);
        Ok((
            dump_normalized_coords(&mut field)
                .into_iter()
                .map(|p| p.clone())
                .collect::<HashSet<Point2D>>(),
            dump_normalized_coords(&mut expected)
                .into_iter()
                .map(|p| p.clone())
                .collect::<HashSet<_>>(),
        ))
    }

    #[test]
    fn example_p1_first_round() {
        let first_round = "\
..............\
\n.......#......\
\n.....#...#....\
\n...#..#.#.....\
\n.......#..#...\
\n....#.#.##....\
\n..#..#.#......\
\n..#.#.#.##....\
\n..............\
\n....#..#......\
\n..............\
\n..............\
";
        let (actual, expected) = compare_for_n_rounds(1, EXAMPLE, first_round).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_p1_second_round() {
        let second_round = "\
..............\
\n.......#......\
\n....#.....#...\
\n...#..#.#.....\
\n.......#...#..\
\n...#..#.#.....\
\n.#...#.#.#....\
\n..............\
\n..#.#.#.##....\
\n....#..#......\
\n..............\
\n..............\
";
        let (actual, expected) = compare_for_n_rounds(2, EXAMPLE, second_round).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_p1_third_round() {
        let third_round = "\
..............\
\n.......#......\
\n.....#....#...\
\n..#..#...#....\
\n.......#...#..\
\n...#..#.#.....\
\n.#..#.....#...\
\n.......##.....\
\n..##.#....#...\
\n...#..........\
\n.......#......\
\n..............\
";
        let (actual, expected) = compare_for_n_rounds(3, EXAMPLE, third_round).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_p1_tenth_round() {
        let tenth_round = "\
.......#......\
\n...........#..\
\n..#.#..#......\
\n......#.......\
\n...#.....#..#.\
\n.#......##....\
\n.....##.......\
\n..#........#..\
\n....#.#..#....\
\n..............\
\n....#..#..#...\
\n..............\
";
        let (actual, expected) = compare_for_n_rounds(10, EXAMPLE, tenth_round).unwrap();
        assert_eq!(actual, expected);
    }

    const MINI_EXAMPLE: &str = ".....\
       \n..##.\
       \n..#..\
       \n.....\
       \n..##.\
       \n.....";

    fn get_mini_example() -> impl BufRead {
        io::BufReader::new(MINI_EXAMPLE.as_bytes())
    }

    #[test]
    fn mini_example_round1() {
        let first_round = "\
..##.\
\n.....\
\n..#..\
\n...#.\
\n..#..\
\n.....\
";
        let (actual, expected) = compare_for_n_rounds(1, MINI_EXAMPLE, first_round).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn mini_example_round2() {
        let second_round = "\
.....\
\n..##.\
\n.#...\
\n....#\
\n.....\
\n..#..\
";
        let (actual, expected) = compare_for_n_rounds(2, MINI_EXAMPLE, second_round).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_elf_proposal() {
        let mut field = Field::parse(get_mini_example()).unwrap();
        field.normalize();
        let mut proposals = HashMap::new();
        let mut impossible_moves = HashSet::new();
        let top_left = Point2D::new(0, 0);
        field.elf_proposal(&top_left, &mut proposals, &mut impossible_moves);
        assert_eq!(
            proposals,
            vec![(Point2D::new(0, -1), Point2D::new(0, 0))]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );
    }

    #[test]
    fn example_p1() {
        let field = Field::parse(get_example_br()).unwrap();
        assert_eq!(problem1(field), 110);
    }

    #[test]
    fn example_p2() {
        let field = Field::parse(get_example_br()).unwrap();
        assert_eq!(problem2(field), 20);
    }
}
