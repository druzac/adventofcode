use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum TetraminoType {
    Horizontal,
    Cross,
    L,
    Vertical,
    Square,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn new(x: i64, y: i64) -> Point2D {
        Point2D { x: x, y: y }
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let y_ord = self.y.cmp(&other.y);
        if y_ord != cmp::Ordering::Equal {
            return y_ord;
        }
        self.x.cmp(&other.x)
    }
}

impl cmp::PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Point2D) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RockShape {
    rock_type: TetraminoType,
    bottom_left: Point2D,
}

impl RockShape {
    fn new(rtype: TetraminoType, bottom_left: Point2D) -> RockShape {
        RockShape {
            rock_type: rtype,
            bottom_left: bottom_left,
        }
    }

    fn with_offsets(
        rtype: TetraminoType,
        x_line: i64,
        x_line_offset: i64,
        y_line: i64,
        y_line_offset: i64,
    ) -> RockShape {
        let y = y_line + y_line_offset;
        let x = match rtype {
            TetraminoType::Horizontal
            | TetraminoType::L
            | TetraminoType::Vertical
            | TetraminoType::Square => x_line + x_line_offset,
            TetraminoType::Cross => x_line + x_line_offset + 1,
        };
        RockShape::new(rtype, Point2D::new(x, y))
    }

    fn points(&self) -> TetraminoPointsIterator {
        TetraminoPointsIterator { idx: 0, rock: self }
    }

    fn apply_wind(&self, jet_dir: JetDirection) -> RockShape {
        let new_x = match jet_dir {
            JetDirection::Left => self.bottom_left.x - 1,
            JetDirection::Right => self.bottom_left.x + 1,
        };
        RockShape::new(self.rock_type, Point2D::new(new_x, self.bottom_left.y))
    }

    fn apply_gravity(&self) -> RockShape {
        let new_y = self.bottom_left.y - 1;
        RockShape::new(self.rock_type, Point2D::new(self.bottom_left.x, new_y))
    }
}

impl Ord for RockShape {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let point_ord = self.bottom_left.cmp(&other.bottom_left);
        if point_ord != cmp::Ordering::Equal {
            return point_ord;
        }
        self.rock_type.cmp(&other.rock_type)
    }
}

impl cmp::PartialOrd for RockShape {
    fn partial_cmp(&self, other: &RockShape) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct TetraminoPointsIterator<'a> {
    idx: usize,
    rock: &'a RockShape,
}

impl<'a> TetraminoPointsIterator<'a> {
    fn next_horizontal(&mut self) -> Option<Point2D> {
        if self.idx > 3 {
            return None;
        }
        let mut result = self.rock.bottom_left.clone();
        result.x += self.idx as i64;
        self.idx += 1;
        Some(result)
    }

    fn next_cross(&mut self) -> Option<Point2D> {
        if self.idx > 4 {
            return None;
        }
        let mut result = self.rock.bottom_left.clone();
        result.y += match self.idx {
            0 => 0,
            1 | 2 | 3 => 1,
            4 => 2,
            _ => panic!(),
        };
        result.x += match self.idx {
            0 | 2 | 4 => 0,
            1 => -1,
            3 => 1,
            _ => panic!(),
        };
        self.idx += 1;
        Some(result)
    }

    fn next_l(&mut self) -> Option<Point2D> {
        if self.idx > 4 {
            return None;
        }
        let mut result = self.rock.bottom_left.clone();
        result.x += cmp::min(self.idx as i64, 2);
        result.y += cmp::max(self.idx as i64 - 2, 0);
        self.idx += 1;
        Some(result)
    }

    fn next_vertical(&mut self) -> Option<Point2D> {
        if self.idx > 3 {
            return None;
        }
        let mut result = self.rock.bottom_left.clone();
        result.y += self.idx as i64;
        self.idx += 1;
        Some(result)
    }

    fn next_square(&mut self) -> Option<Point2D> {
        if self.idx > 3 {
            return None;
        }
        let mut result = self.rock.bottom_left.clone();
        result.x += if self.idx % 2 == 1 { 1 } else { 0 };
        result.y += if self.idx > 1 { 1 } else { 0 };
        self.idx += 1;
        Some(result)
    }
}

impl<'a> Iterator for TetraminoPointsIterator<'a> {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rock.rock_type {
            TetraminoType::Horizontal => self.next_horizontal(),
            TetraminoType::Cross => self.next_cross(),
            TetraminoType::L => self.next_l(),
            TetraminoType::Vertical => self.next_vertical(),
            TetraminoType::Square => self.next_square(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum JetDirection {
    Left,
    Right,
}

impl JetDirection {
    fn from_char(c: char) -> Result<JetDirection, AOCError> {
        match c {
            '<' => Ok(JetDirection::Left),
            '>' => Ok(JetDirection::Right),
            _ => Err(aocerror!("unexpected character: {}", c)),
        }
    }
}

struct WindSource {
    idx: usize,
    wind_pattern: Vec<JetDirection>,
}

impl WindSource {
    fn new(wind_pattern: Vec<JetDirection>) -> WindSource {
        WindSource {
            idx: 0,
            wind_pattern: wind_pattern,
        }
    }

    fn next_jet(&mut self) -> JetDirection {
        let result = self.wind_pattern[self.idx];
        self.idx = (self.idx + 1) % self.wind_pattern.len();
        result
    }
}

impl FromStr for WindSource {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<WindSource, Self::Err> {
        s.chars()
            .map(|c| JetDirection::from_char(c))
            .collect::<Result<Vec<JetDirection>, AOCError>>()
            .map(|v| WindSource::new(v))
    }
}

struct RockSource {
    cnt: usize,
}

impl RockSource {
    fn new() -> RockSource {
        RockSource { cnt: 0 }
    }

    fn next_rock_type(&mut self) -> TetraminoType {
        let result = match self.cnt % 5 {
            0 => TetraminoType::Horizontal,
            1 => TetraminoType::Cross,
            2 => TetraminoType::L,
            3 => TetraminoType::Vertical,
            4 => TetraminoType::Square,
            _ => panic!(),
        };
        self.cnt = (self.cnt + 1) % 5;
        result
    }
}

// |..@@@@.|
// |.......|
// |.......|
// |.......|
// +-------+
const CAVERN_LEFT_EDGE: i64 = 0;
const CAVERN_RIGHT_EDGE: i64 = 8;
const CAVERN_FLOOR: i64 = 0;
const FALLING_ROCK_Y_OFFSET: i64 = 4;
const FALLING_ROCK_X_OFFSET: i64 = 3;

struct Cavern {
    settled_rocks: HashSet<Point2D>,
    wind_source: WindSource,
    rock_source: RockSource,
    highest_y: i64,
}

impl Cavern {
    fn new(wind_source: WindSource) -> Cavern {
        Cavern {
            settled_rocks: HashSet::new(),
            wind_source: wind_source,
            rock_source: RockSource::new(),
            highest_y: CAVERN_FLOOR,
        }
    }

    fn intersects_cavern(&self, rock: &RockShape) -> bool {
        for point in rock.points() {
            if point.x == CAVERN_LEFT_EDGE
                || point.x == CAVERN_RIGHT_EDGE
                || point.y == CAVERN_FLOOR
            {
                return true;
            }
        }
        false
    }

    fn intersects_settled_rock(&self, rock: &RockShape) -> bool {
        for point in rock.points() {
            if self.settled_rocks.contains(&point) {
                return true;
            }
        }
        false
    }

    fn valid_rock_shape(&self, rock: &RockShape) -> bool {
        !self.intersects_cavern(rock) && !self.intersects_settled_rock(rock)
    }

    fn next_falling_rock(&mut self) -> RockShape {
        let rtype = self.rock_source.next_rock_type();
        RockShape::with_offsets(
            rtype,
            CAVERN_LEFT_EDGE,
            FALLING_ROCK_X_OFFSET,
            self.highest_y,
            FALLING_ROCK_Y_OFFSET,
        )
    }

    fn settle_rock(&mut self, rock: RockShape) -> bool {
        let mut collision = false;
        for point in rock.points() {
            self.highest_y = cmp::max(self.highest_y, point.y);
            collision |= self.settled_rocks.insert(point);
        }
        collision
    }

    fn add_rock(&mut self) {
        let mut next_rs = self.next_falling_rock();
        loop {
            let blown_rs = next_rs.apply_wind(self.wind_source.next_jet());
            if self.valid_rock_shape(&blown_rs) {
                next_rs = blown_rs;
            }
            let fallen_rs = next_rs.apply_gravity();
            if self.valid_rock_shape(&fallen_rs) {
                next_rs = fallen_rs;
            } else {
                assert!(self.settle_rock(next_rs));
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn draw_with_falling_rock(&self, maybe_rock: Option<&RockShape>) {
        println!();
        let mut falling_rock = HashSet::new();
        let mut max_y = None;
        if let Some(rock) = maybe_rock {
            for point in rock.points() {
                assert!(falling_rock.insert(point.clone()));
                max_y = if let Some(curr_max_y) = max_y {
                    Some(cmp::max(curr_max_y, point.y))
                } else {
                    Some(point.y)
                };
            }
        }
        let concrete_max_y = max_y.unwrap_or(CAVERN_FLOOR + 3);
        for y in (0..(concrete_max_y + 1)).rev() {
            for x in CAVERN_LEFT_EDGE..(CAVERN_RIGHT_EDGE + 1) {
                let current_point = Point2D::new(x, y);
                if (x == CAVERN_LEFT_EDGE || x == CAVERN_RIGHT_EDGE) && y == CAVERN_FLOOR {
                    print!("+");
                } else if x == CAVERN_LEFT_EDGE || x == CAVERN_RIGHT_EDGE {
                    print!("|");
                } else if y == CAVERN_FLOOR {
                    print!("-");
                } else if falling_rock.contains(&current_point) {
                    print!("@");
                } else if self.settled_rocks.contains(&current_point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    #[allow(dead_code)]
    fn find_criss_crosses(&self) -> Vec<i64> {
        let mut results = Vec::new();
        for y in 1..(self.highest_y + 1) {
            let mut criss_cross = true;
            for x in CAVERN_LEFT_EDGE + 1..CAVERN_RIGHT_EDGE {
                if !self.settled_rocks.contains(&Point2D::new(x, y))
                    && !self.settled_rocks.contains(&Point2D::new(x, y + 1))
                {
                    criss_cross = false;
                    break;
                }
            }
            if criss_cross {
                results.push(y);
            }
        }
        results
    }
}

fn parse_problem<B: io::BufRead>(mut br: B) -> Result<WindSource, AOCError> {
    let mut buf = String::new();
    br.read_line(&mut buf)?;
    buf.as_str().trim().parse::<WindSource>()
}

fn problem1(wind_source: WindSource) -> u64 {
    let mut cavern = Cavern::new(wind_source);
    for _ in 0..2022 {
        cavern.add_rock();
    }
    assert!(cavern.highest_y >= 0);
    cavern.highest_y as u64
}

fn problem2(_: WindSource) -> u64 {
    panic!("unimplemented")
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let problem = parse_problem(br)?;
    let result = match part {
        ProblemPart::P1 => problem1(problem),
        ProblemPart::P2 => problem2(problem),
    };
    println!("result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_JETS: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn with_offsets_cross() {
        let rs = RockShape::with_offsets(TetraminoType::Cross, 0, 1, 0, 1);
        assert_eq!(rs.bottom_left, Point2D::new(2, 1));
    }

    #[test]
    fn with_offsets_horizontal() {
        let rs = RockShape::with_offsets(TetraminoType::Horizontal, 0, 1, 0, 1);
        assert_eq!(rs.bottom_left, Point2D::new(1, 1));
    }

    fn get_example_wind_source() -> WindSource {
        WindSource::new(
            EXAMPLE_JETS
                .chars()
                .map(|c| JetDirection::from_char(c))
                .collect::<Result<Vec<JetDirection>, AOCError>>()
                .unwrap(),
        )
    }

    #[test]
    fn example_single_rock() {
        let mut cavern = Cavern::new(get_example_wind_source());
        cavern.add_rock();
        cavern.add_rock();
        cavern.add_rock();
        let expected = vec![
            RockShape::new(TetraminoType::Horizontal, Point2D::new(3, 1)),
            RockShape::new(TetraminoType::Cross, Point2D::new(4, 2)),
            RockShape::new(TetraminoType::L, Point2D::new(1, 4)),
        ]
        .iter()
        .flat_map(|rs| rs.points())
        .collect::<HashSet<_>>();
        assert_eq!(cavern.settled_rocks, expected,);
    }

    #[test]
    fn example_full() {
        let mut cavern = Cavern::new(get_example_wind_source());
        for _ in 0..2022 {
            cavern.add_rock();
        }
        assert_eq!(cavern.highest_y, 3068);
    }
}
