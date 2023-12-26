use crate::{AOCError, ProblemPart};

use std::cmp;
use std::collections::HashSet;
use std::convert::From;
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn man_distance(self: &Point, rhs: &Point) -> u64 {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as u64
    }

    fn tuning_frequency(self: &Point) -> i64 {
        self.x * 4000000 + self.y
    }
}

struct SensorWithBeacon {
    sensor: Point,
    beacon: Point,
}

fn parse_number_word(word: &str) -> Result<i64, AOCError> {
    let suffixes: &[_] = &[',', ':'];
    Ok(word[2..].trim_end_matches(suffixes).parse::<i64>()?)
}

impl FromStr for SensorWithBeacon {
    type Err = AOCError;

    // Sensor at x=2793338, y=1910659: closest beacon is at x=2504930, y=2301197
    fn from_str(s: &str) -> Result<SensorWithBeacon, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();
        let sensor = Point {
            x: parse_number_word(words[2])?,
            y: parse_number_word(words[3])?,
        };
        let beacon = Point {
            x: parse_number_word(words[8])?,
            y: parse_number_word(words[9])?,
        };
        Ok(SensorWithBeacon {
            sensor: sensor,
            beacon: beacon,
        })
    }
}

#[derive(Debug)]
struct HorizontalLine {
    y: i64,
}

impl HorizontalLine {
    fn man_distance(self: &HorizontalLine, rhs: &Point) -> u64 {
        (rhs.y - self.y).abs() as u64
    }
}

#[derive(Debug, Clone)]
struct Interval {
    a: i64,
    b: i64,
}

impl Interval {
    fn have_intersection(&self, rhs: &Interval) -> bool {
        if self.a <= rhs.a {
            rhs.a <= self.b
        } else {
            self.a <= rhs.b
        }
    }

    fn merge_intervals(&self, rhs: &Interval) -> Option<Interval> {
        if self.have_intersection(rhs) {
            Some(Interval {
                a: cmp::min(self.a, rhs.a),
                b: cmp::max(self.b, rhs.b),
            })
        } else {
            None
        }
    }

    fn new(a: i64, b: i64) -> Interval {
        Interval {
            a: cmp::min(a, b),
            b: cmp::max(a, b),
        }
    }

    fn contains(&self, val: i64) -> bool {
        self.a <= val && val <= self.b
    }

    fn len(&self) -> u64 {
        (1 + self.b - self.a) as u64
    }
}

#[derive(Debug)]
struct Circle {
    centre: Point,
    r: u64,
}

impl Circle {
    fn new(centre: Point, radius: u64) -> Circle {
        Circle {
            centre: centre,
            r: radius,
        }
    }
}

impl From<&SensorWithBeacon> for Circle {
    fn from(val: &SensorWithBeacon) -> Self {
        Circle::new(val.sensor.clone(), val.sensor.man_distance(&val.beacon))
    }
}

fn intersect(l: &HorizontalLine, c: &Circle) -> Option<Interval> {
    let line_to_centre_distance = l.man_distance(&c.centre);
    if line_to_centre_distance > c.r {
        return None;
    }
    let delta = (c.r - line_to_centre_distance) as i64;
    Some(Interval::new(c.centre.x - delta, c.centre.x + delta))
}

fn add_to_disjoint_intervals(mut intervals: Vec<Interval>, mut to_add: Interval) -> Vec<Interval> {
    let mut i = 0;
    while i < intervals.len() {
        if let Some(merged) = to_add.merge_intervals(&intervals[i]) {
            to_add = merged;
            // no need to increment i because i refers to a different element now.
            intervals.swap_remove(i);
        } else {
            i += 1;
        }
    }
    intervals.push(to_add);
    intervals
}

fn blocked_segments_on_line(sbs: &[SensorWithBeacon], line: &HorizontalLine) -> Vec<Interval> {
    let mut disjoint_intervals = Vec::with_capacity(sbs.len());
    for sensor_beacon in sbs {
        if let Some(ival) = intersect(&line, &Circle::from(sensor_beacon)) {
            disjoint_intervals = add_to_disjoint_intervals(disjoint_intervals, ival)
        }
    }
    disjoint_intervals
}

fn problem1(sbs: &[SensorWithBeacon]) -> i64 {
    let line = HorizontalLine { y: 2000000 };
    let intervals = blocked_segments_on_line(sbs, &line);
    let mut result = 0;
    for interval in &intervals {
        let mut hits = HashSet::new();
        for sb in sbs {
            if line.y == sb.sensor.y && interval.contains(sb.sensor.x) {
                hits.insert(sb.sensor.x);
            }
            if line.y == sb.beacon.y && interval.contains(sb.beacon.x) {
                hits.insert(sb.beacon.x);
            }
        }
        result += (interval.len() - (hits.len() as u64)) as i64;
    }
    result
}

fn interval_min_max(interval1: Interval, interval2: Interval) -> (Interval, Interval) {
    if interval1.a < interval2.a {
        (interval1, interval2)
    } else {
        (interval2, interval1)
    }
}

fn problem2(problem: &[SensorWithBeacon]) -> i64 {
    // brute force all lines
    for y in 0..4000000 {
        let mut intervals = blocked_segments_on_line(problem, &HorizontalLine { y: y });
        if intervals.len() == 2 {
            let (first, second) =
                interval_min_max(intervals.swap_remove(1), intervals.swap_remove(0));
            let x_candidate = first.b + 1;
            assert!(x_candidate < second.a && x_candidate >= 0 && x_candidate <= 4000000);
            let p = Point {
                x: x_candidate,
                y: y,
            };
            return p.tuning_frequency();
        } else {
            let interval = &intervals[0];
            if 0 < interval.a || interval.b < 4000000 {
                println!("candidate point? {:?}", interval);
            }
        }
    }
    panic!();
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let problem: Vec<SensorWithBeacon> = br
        .lines()
        .map(|res_l| res_l?.parse::<SensorWithBeacon>())
        .collect::<Result<Vec<_>, _>>()?;

    let result = match part {
        ProblemPart::P1 => problem1(&problem),
        ProblemPart::P2 => problem2(&problem),
    };
    println!("result: {}", result);
    Ok(())
}
