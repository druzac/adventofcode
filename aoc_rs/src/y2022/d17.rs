use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

const CAVERN_LEFT_EDGE: i64 = 0;
const CAVERN_RIGHT_EDGE: i64 = 8;
const CAVERN_FLOOR: i64 = 0;
const FALLING_ROCK_Y_OFFSET: i64 = 4;
const FALLING_ROCK_X_OFFSET: i64 = 3;

#[derive(Debug)]
struct SettledRocksVecs {
    settled_rocks: [NaturalNumberSet; 7],
    last_reaped_y: u64,
    highest_y: u64,
}

impl SettledRocksVecs {
    fn new() -> SettledRocksVecs {
        SettledRocksVecs {
            settled_rocks: [
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
                NaturalNumberSet::new(),
            ],
            last_reaped_y: 0,
            highest_y: 0,
        }
    }

    fn point_to_coords(&self, point: &Point2D) -> Option<(usize, u64)> {
        if point.x <= CAVERN_LEFT_EDGE || point.x >= CAVERN_RIGHT_EDGE || point.y < 0 {
            return None;
        }
        let idx = (point.x - (CAVERN_LEFT_EDGE + 1)) as usize;
        Some((idx, point.y as u64))
    }

    fn prune_unreachable_depths(&mut self, start: u64, end: u64) {
        for y in (start..end).rev() {
            if self.is_passable_horizontal_line(y) {
                continue;
            }
            self.last_reaped_y = y as u64;
            for sr in self.settled_rocks.iter_mut() {
                sr.remove_prefix(self.last_reaped_y);
            }
            return;
        }
    }

    fn is_passable_horizontal_line(&self, y_val: u64) -> bool {
        for x in CAVERN_LEFT_EDGE + 1..CAVERN_RIGHT_EDGE {
            if !self.contains(&Point2D::new(x, y_val as i64))
                && !self.contains(&Point2D::new(x, (y_val + 1) as i64))
            {
                return true;
            }
        }
        false
    }

    fn get_signature<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for sr in self.settled_rocks.iter() {
            sr.get_signature(state);
        }
    }

    fn intersects_settled_rock(&self, rs: &RockShape) -> bool {
        if rs.bottom_left.y > self.highest_y as i64 {
            return false;
        }
        for point in rs.points() {
            if self.contains(&point) {
                return true;
            }
        }
        false
    }

    fn highest_y(&self) -> i64 {
        self.highest_y as i64
    }

    fn contains(&self, point: &Point2D) -> bool {
        if point.x <= CAVERN_LEFT_EDGE || point.x >= CAVERN_RIGHT_EDGE || point.y <= 0 {
            return false;
        }
        let idx = (point.x - (CAVERN_LEFT_EDGE + 1)) as usize;
        self.settled_rocks[idx].contains(point.y as u64)
    }

    fn settle_rock(&mut self, rs: RockShape) -> bool {
        let mut collision = false;
        let mut lowest_added_y = u64::MAX;
        let mut highest_added_y = u64::MIN;
        for point in rs.points() {
            assert!(point.y >= 0);
            lowest_added_y = cmp::min(lowest_added_y, point.y as u64);
            highest_added_y = cmp::max(highest_added_y, point.y as u64);
            let (idx, y) = self.point_to_coords(&point).unwrap();
            collision |= self.settled_rocks[idx].insert(y);
        }
        self.highest_y = cmp::max(self.highest_y, highest_added_y);
        if lowest_added_y < self.last_reaped_y {
            println!(
                "settled rock: {:?}, points: {:?}",
                rs,
                rs.points().collect::<Vec<_>>()
            );

            panic!("oops, something broke");
        }
        self.prune_unreachable_depths(lowest_added_y, highest_added_y + 1);
        collision
    }
}

#[derive(Debug)]
struct NaturalNumberSet {
    nums: Vec<u64>,
    offset: u64,
}

impl NaturalNumberSet {
    fn new() -> NaturalNumberSet {
        NaturalNumberSet {
            nums: Vec::new(),
            offset: 0,
        }
    }

    fn contains(&self, n: u64) -> bool {
        let (idx, mask) = self.num_to_repr(n);
        if idx >= self.nums.len() {
            return false;
        }
        self.nums[idx] & mask != 0
    }

    fn remove_prefix(&mut self, mut cutoff: u64) {
        let rem = cutoff % 64;
        if rem != 0 {
            cutoff -= rem;
        }
        let lowest_idx_to_keep = self.num_to_repr(cutoff).0;
        let new_nums = self.nums[lowest_idx_to_keep..].to_vec();
        self.nums = new_nums;
        self.offset = cutoff;
    }

    fn insert(&mut self, n: u64) -> bool {
        let (idx, mask) = self.num_to_repr(n);
        while idx >= self.nums.len() {
            self.nums.push(0);
        }
        if self.nums[idx] & mask != 0 {
            return false;
        }
        self.nums[idx] |= mask;
        true
    }

    fn num_to_repr(&self, n: u64) -> (usize, u64) {
        assert!(n >= self.offset);
        let offset_n = n - self.offset;
        ((offset_n / 64) as usize, 1 << (offset_n % 64))
    }

    fn get_signature<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.nums.hash(state)
    }
}

struct Cavern {
    settled_rocks: SettledRocksVecs,
    wind_source: WindSource,
    rock_source: RockSource,
    added_rocks: usize,
}

impl Cavern {
    fn new(wind_source: WindSource) -> Cavern {
        Cavern {
            settled_rocks: SettledRocksVecs::new(),
            wind_source: wind_source,
            rock_source: RockSource::new(),
            added_rocks: 0,
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

    fn valid_rock_shape(&self, rock: &RockShape) -> bool {
        !self.intersects_cavern(rock) && !self.settled_rocks.intersects_settled_rock(rock)
    }

    fn next_falling_rock(&mut self) -> RockShape {
        let rtype = self.rock_source.next_rock_type();
        RockShape::with_offsets(
            rtype,
            CAVERN_LEFT_EDGE,
            FALLING_ROCK_X_OFFSET,
            self.settled_rocks.highest_y(),
            FALLING_ROCK_Y_OFFSET,
        )
    }

    fn add_rock(&mut self) {
        self.added_rocks += 1;
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
                assert!(self.settled_rocks.settle_rock(next_rs));
                break;
            }
        }
    }

    #[allow(dead_code)]
    fn draw_with_falling_rock(&self, maybe_rock: Option<&RockShape>) {
        println!();
        let mut falling_rock = HashSet::new();
        let max_y = cmp::max(self.settled_rocks.highest_y(), CAVERN_FLOOR + 3);
        if let Some(rock) = maybe_rock {
            for point in rock.points() {
                assert!(falling_rock.insert(point.clone()));
            }
        }
        for y in (0..(max_y + 1)).rev() {
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

    fn get_signature(&self) -> u64 {
        let mut state = DefaultHasher::new();
        state.write_usize(self.wind_source.idx);
        state.write_usize(self.rock_source.cnt);
        self.settled_rocks.get_signature(&mut state);
        state.finish()
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
    let highest_y = cavern.settled_rocks.highest_y();
    assert!(highest_y >= 0);
    highest_y as u64
}

fn cycle_search(cavern: &mut Cavern, max_val: usize) -> (usize, i64) {
    let mut states = HashMap::new();
    for _ in 0..max_val {
        let value = (cavern.added_rocks, cavern.settled_rocks.highest_y());
        let key = cavern.get_signature();
        if let Some(existing_value) = states.insert(key, value) {
            return (value.0 - existing_value.0, value.1 - existing_value.1);
        }
        cavern.add_rock();
    }
    panic!("no cycle found!");
}

fn problem2(wind_source: WindSource) -> u64 {
    let mut cavern = Cavern::new(wind_source);
    let target = 1000000000000 as usize;
    let (cycle_length, added_height) = cycle_search(&mut cavern, target);
    while (target - cavern.added_rocks) % cycle_length != 0 {
        cavern.add_rock()
    }
    let number_of_cycles = (target - cavern.added_rocks) / cycle_length;
    (cavern.settled_rocks.highest_y() + (number_of_cycles as i64) * added_height) as u64
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
    fn example_full() {
        let mut cavern = Cavern::new(get_example_wind_source());
        for _ in 0..2022 {
            cavern.add_rock();
        }
        assert_eq!(cavern.settled_rocks.highest_y, 3068);
    }

    #[test]
    fn nn_set_contains_high() {
        let nnset = NaturalNumberSet::new();
        assert!(!nnset.contains(100));
    }

    #[test]
    fn nn_set_insert_small() {
        let mut nnset = NaturalNumberSet::new();
        assert!(nnset.insert(0));
        assert!(nnset.contains(0));
        assert!(!nnset.contains(100));
    }

    #[test]
    fn nn_set_insert_bounds() {
        let mut nnset = NaturalNumberSet::new();
        assert!(nnset.insert(64));
        assert!(nnset.contains(64));
        assert!(!nnset.contains(63));
        assert!(!nnset.contains(65));
    }

    #[test]
    fn nn_pruning() {
        let mut nnset = NaturalNumberSet::new();
        nnset.insert(0);
        nnset.insert(1);
        nnset.insert(65);
        nnset.insert(500);
        nnset.remove_prefix(64);
        assert!(nnset.contains(65));
        assert!(nnset.contains(500));
    }

    #[test]
    fn nn_repeat_pruning() {
        let mut nnset = NaturalNumberSet::new();
        nnset.insert(0);
        nnset.insert(1);
        nnset.insert(65);
        nnset.insert(129);
        nnset.remove_prefix(64);
        assert!(nnset.contains(65));
        assert!(nnset.contains(129));
        nnset.remove_prefix(128);
        assert!(nnset.contains(129));
    }
}
