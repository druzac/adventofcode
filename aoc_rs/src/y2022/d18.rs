use crate::{aocerror, AOCError, ProblemPart};

use std::cmp::{self, Ordering};
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, Lines};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

struct Point3DNeighbourIterator {
    points: [Point3D; 6],
    idx: usize,
}

impl PartialOrd for Point3D {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let x_cmp = self.x.cmp(&rhs.x);
        let y_cmp = self.y.cmp(&rhs.y);
        let z_cmp = self.z.cmp(&rhs.z);
        if x_cmp == y_cmp && y_cmp == z_cmp {
            Some(x_cmp)
        } else {
            None
        }
    }
}

impl Iterator for Point3DNeighbourIterator {
    type Item = Point3D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= 6 {
            None
        } else {
            let result = Some(self.points[self.idx]);
            self.idx += 1;
            result
        }
    }
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }

    fn get_neighbours(&self) -> impl Iterator<Item = Point3D> {
        let mut neighbours: [Point3D; 6] = [*self; 6];
        neighbours[0].x += 1;
        neighbours[1].x -= 1;
        neighbours[2].y += 1;
        neighbours[3].y -= 1;
        neighbours[4].z += 1;
        neighbours[5].z -= 1;
        Point3DNeighbourIterator {
            points: neighbours,
            idx: 0,
        }
    }

    fn pointwise_min(&self, rhs: &Point3D) -> Point3D {
        Point3D::new(
            cmp::min(self.x, rhs.x),
            cmp::min(self.y, rhs.y),
            cmp::min(self.z, rhs.z),
        )
    }

    fn pointwise_max(&self, rhs: &Point3D) -> Point3D {
        Point3D::new(
            cmp::max(self.x, rhs.x),
            cmp::max(self.y, rhs.y),
            cmp::max(self.z, rhs.z),
        )
    }
}

struct Cuboid {
    min_point: Point3D,
    max_point: Point3D,
}

impl Cuboid {
    fn contains(&self, point: &Point3D) -> bool {
        &self.min_point <= point && point <= &self.max_point
    }
}

impl FromStr for Point3D {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Point3D { x: 0, y: 0, z: 0 };
        for (i, subs) in s.split(',').enumerate() {
            match i {
                0 => result.x = subs.parse::<i64>()?,
                1 => result.y = subs.parse::<i64>()?,
                2 => result.z = subs.parse::<i64>()?,
                _ => return Err(aocerror!("too many commas in string: {}", s)),
            }
        }
        Ok(result)
    }
}

fn parse_problem<B: BufRead>(lines: Lines<B>) -> Result<HashSet<Point3D>, AOCError> {
    lines.map(|s| s?.parse::<Point3D>()).collect()
}

fn surface_area(points: &HashSet<Point3D>) -> u64 {
    points
        .iter()
        .flat_map(|point| point.get_neighbours())
        .filter(|point| !points.contains(point))
        .count() as u64
}

fn problem1(points: HashSet<Point3D>) -> u64 {
    surface_area(&points)
}

struct AirComponent {
    is_outside: bool,
    faces_with_lava: u64,
}

fn find_connected_component(
    lava_points: &HashSet<Point3D>,
    cuboid: &Cuboid,
    global_unexplored: &mut HashSet<Point3D>,
    start_node: Point3D,
) -> AirComponent {
    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);
    let mut explored = HashSet::new();
    explored.insert(start_node);
    let mut outside_component = false;
    let mut lava_faces = 0;
    while let Some(current_point) = frontier.pop_front() {
        for neighbour in current_point.get_neighbours() {
            if explored.contains(&neighbour) {
                continue;
            }
            if lava_points.contains(&neighbour) {
                lava_faces += 1;
                continue;
            }
            if !cuboid.contains(&neighbour) {
                outside_component = true;
                continue;
            }
            explored.insert(neighbour);
            frontier.push_back(neighbour);
            global_unexplored.remove(&neighbour);
        }
    }
    AirComponent {
        is_outside: outside_component,
        faces_with_lava: lava_faces,
    }
}

fn problem2(lava_points: HashSet<Point3D>) -> u64 {
    let first = *lava_points.iter().next().unwrap();
    let min_point = lava_points
        .iter()
        .fold(first, |lhs, rhs| lhs.pointwise_min(rhs));
    let max_point = lava_points
        .iter()
        .fold(first, |lhs, rhs| lhs.pointwise_max(rhs));
    let mut remaining_points = HashSet::new();
    // reduce search space - don't add any neighbours of cubes outside the box.
    for i in min_point.x + 1..max_point.x {
        for j in min_point.y + 1..max_point.y {
            for k in min_point.z + 1..max_point.z {
                let point = Point3D::new(i, j, k);
                if !lava_points.contains(&point) {
                    remaining_points.insert(Point3D::new(i, j, k));
                }
            }
        }
    }
    let bd_cuboid = Cuboid {
        min_point: min_point,
        max_point: max_point,
    };
    let mut internal_faces = 0;
    while let Some(&start_point) = remaining_points.iter().next().clone() {
        remaining_points.remove(&start_point);
        let comp =
            find_connected_component(&lava_points, &bd_cuboid, &mut remaining_points, start_point);
        if !comp.is_outside {
            internal_faces += comp.faces_with_lava;
        }
    }
    surface_area(&lava_points) - internal_faces
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let points: HashSet<_> = parse_problem(br.lines())?;
    let result = match part {
        ProblemPart::P1 => problem1(points),
        ProblemPart::P2 => problem2(points),
    };
    println!("result: {}", result);
    Ok(())
}
