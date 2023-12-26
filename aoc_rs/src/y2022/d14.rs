use crate::{aocerror, AOCError, ProblemPart};

use std::cmp;
use std::fmt;
use std::io;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

// simple line for now
#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

struct LineIterator {
    curr_point: Point,
    delta: Point,
    // end in a cpp sense, when curr_point == end we're done.
    end: Point,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_point == self.end {
            None
        } else {
            let result = self.curr_point;
            self.curr_point += self.delta;
            Some(result)
        }
    }
}

impl Line {
    fn new(a: Point, b: Point) -> Line {
        Line {
            a: cmp::min(a, b),
            b: cmp::max(a, b),
        }
    }

    fn points(&self) -> LineIterator {
        let delta = if self.a.x == self.b.x {
            Point::new(0, 1)
        } else if self.a.y == self.b.y {
            Point::new(1, 0)
        } else {
            panic!()
        };
        LineIterator {
            curr_point: self.a,
            delta: delta,
            end: &self.b + &delta,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    fn pointwise_min(&self, rhs: &Point) -> Point {
        Point::new(cmp::min(self.x, rhs.x), cmp::min(self.y, rhs.y))
    }

    fn pointwise_max(&self, rhs: &Point) -> Point {
        Point::new(cmp::max(self.x, rhs.x), cmp::max(self.y, rhs.y))
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> Sub<&'b Point> for &'b Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

// #[derive(Debug)]
// struct ParseError;

// impl From<ParseIntError> for ParseError {
//     fn from(_: ParseIntError) -> Self {
//         ParseError {}
//     }
// }

// impl From<io::Error> for ParseError {
//     fn from(_: io::Error) -> Self {
//         ParseError {}
//     }
// }

impl FromStr for Point {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // expect: "x,y"
        let nums: Vec<_> = s.split(',').collect();
        if nums.len() != 2 {
            aocerror!("Unexpected string: {}", s);
        }
        Ok(Point::new(
            nums[0].parse::<usize>()?,
            nums[1].parse::<usize>()?,
        ))
    }
}

fn parse_line(line: &str) -> Result<Vec<Line>, AOCError> {
    let points = line
        .split("->")
        .map(|s| s.trim().parse::<Point>())
        .collect::<Result<Vec<_>, AOCError>>()?;
    if points.len() <= 1 {
        return Err(aocerror!("Unexpected number of points on line: {}", line));
    }
    let mut results = Vec::new();
    for idx in 0..(points.len() - 1) {
        results.push(Line::new(points[idx], points[idx + 1]))
    }
    Ok(results)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Tile::Rock => '#',
            Tile::Air => '.',
            Tile::Sand => 'o',
        };
        write!(f, "{}", ch)
    }
}

struct Cave {
    // grid everything in the cave, rock or sand.
    grid: Vec<Vec<Tile>>,
    // where sand comes from
    sand_origin: Point,
}

impl Cave {
    fn new(rock_lines: &[Line], raw_sand_origin: Point) -> Cave {
        let (point_min, point_max) = Self::get_bounds(rock_lines, &raw_sand_origin);
        let size_vector = &point_max - &point_min + Point::new(1, 1);
        let mut rows: Vec<Vec<Tile>> = Vec::with_capacity(size_vector.y);
        for _ in 0..size_vector.y {
            rows.push(vec![Tile::Air; size_vector.x]);
        }
        for rock_line in rock_lines {
            for point in rock_line.points() {
                let translated_point = &point - &point_min;
                rows[translated_point.y][translated_point.x] = Tile::Rock;
            }
        }
        Cave {
            grid: rows,
            sand_origin: &raw_sand_origin - &point_min,
        }
    }

    fn get_bounds(rock_lines: &[Line], raw_sand_origin: &Point) -> (Point, Point) {
        let mut point_min = *raw_sand_origin;
        let mut point_max = *raw_sand_origin;
        for line in rock_lines {
            point_min = point_min.pointwise_min(&line.a);
            point_max = point_max.pointwise_max(&line.b);
        }
        assert!(point_max.x >= point_min.x);
        assert!(point_max.y >= point_min.y);
        (point_min, point_max)
    }

    fn tile_at_point(&self, point: &Point) -> Tile {
        self.grid[point.y][point.x]
    }

    fn mark_sand_at_point(&mut self, point: &Point) {
        self.grid[point.y][point.x] = Tile::Sand
    }

    fn points_below(&self, point: &Point) -> Vec<Point> {
        let mut results = Vec::new();
        if point.y + 1 >= self.grid.len() {
            return results;
        }
        let below_point = point + &Point::new(0, 1);
        results.push(below_point);
        if point.x == 0 {
            return results;
        }
        results.push(&below_point - &Point::new(1, 0));
        if point.x + 1 >= self.grid[below_point.y].len() {
            return results;
        }
        results.push(below_point + Point::new(1, 0));
        results
    }

    fn add_sand(&mut self) -> bool {
        let mut sand_pos = self.sand_origin;
        let current_tile = self.tile_at_point(&sand_pos);
        assert!(current_tile != Tile::Rock);
        if current_tile == Tile::Sand {
            return false;
        }
        loop {
            let points_below = self.points_below(&sand_pos);
            if let Some(new_point) = points_below
                .iter()
                .find(|p| self.tile_at_point(p) == Tile::Air)
            {
                sand_pos = *new_point;
                continue;
            }
            // 3 supports, none air. we stop falling
            if points_below.len() == 3 {
                self.mark_sand_at_point(&sand_pos);
                return true;
            }
            // we don't have 3 supports. we fall forever.
            return false;
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if x == self.sand_origin.x as usize
                    && y == self.sand_origin.y as usize
                    && self.grid[y][x] == Tile::Air
                {
                    write!(f, "+",)?;
                } else {
                    write!(f, "{}", self.grid[y][x])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn problem1(all_lines: &[Line]) -> u64 {
    let mut cave = Cave::new(all_lines, Point::new(500, 0));
    let mut sum = 0;
    while cave.add_sand() {
        sum += 1;
    }
    sum
}

fn problem2(mut all_lines: Vec<Line>) -> u64 {
    let sand_origin = Point::new(500, 0);
    let (point_min, point_max) = Cave::get_bounds(&all_lines, &sand_origin);
    let y_delta = point_max.y + 5;
    assert!(point_min.x >= y_delta);
    let bottom_line = Line {
        a: Point::new(point_min.x - y_delta, point_max.y + 2),
        b: Point::new(point_max.x + y_delta, point_max.y + 2),
    };
    all_lines.push(bottom_line);
    problem1(&all_lines)
}

fn parse_problem<B: io::BufRead>(br: B) -> Result<Vec<Line>, AOCError> {
    let mut all_lines = Vec::new();
    for res_l in br.lines() {
        let line = res_l?;
        for parsed_line in parse_line(&line)? {
            all_lines.push(parsed_line);
        }
    }
    Ok(all_lines)
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let all_lines = parse_problem(br)?;
    match part {
        ProblemPart::P1 => println!("{}", problem1(&all_lines)),
        ProblemPart::P2 => println!("{}", problem2(all_lines)),
    };
    Ok(())
}
