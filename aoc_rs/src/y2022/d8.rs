use crate::{AOCError, ProblemPart};

use std::collections::HashSet;
use std::io;

struct Coordinate {
    height: u32,
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(height: u32, row: usize, column: usize) -> Coordinate {
        Coordinate {
            height: height,
            row: row,
            column: column,
        }
    }
}

fn parse_grid(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_col(grid: &[Vec<u32>], col_idx: usize) -> Vec<Coordinate> {
    let mut result = Vec::new();
    for row in 0..grid.len() {
        result.push(Coordinate::new(grid[row][col_idx], row, col_idx));
    }
    result
}

fn get_row(grid: &[Vec<u32>], row_idx: usize) -> Vec<Coordinate> {
    grid[row_idx]
        .iter()
        .enumerate()
        .map(|(col_idx, height)| Coordinate::new(*height, row_idx, col_idx))
        .collect()
}

fn add_visible(sight_line: &[Coordinate], marks: &mut HashSet<(usize, usize)>) {
    let mut curr_max = None;
    for coord in sight_line {
        if let Some(h) = curr_max {
            if h >= coord.height {
                continue;
            }
        }
        curr_max = Some(coord.height);
        marks.insert((coord.row, coord.column));
    }
}

fn cast_col_ray(grid: &[Vec<u32>], row: usize, col: usize) -> (Vec<u32>, Vec<u32>) {
    let mut up = Vec::new();
    if row > 0 {
        for i in 0..row {
            up.push(grid[i][col]);
        }
        up.reverse();
    }
    let mut down = Vec::new();
    for i in row + 1..grid.len() {
        down.push(grid[i][col])
    }
    (up, down)
}

fn cast_row_ray(grid: &[Vec<u32>], row: usize, col: usize) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    if col > 0 {
        for j in 0..col {
            left.push(grid[row][j]);
        }
        left.reverse();
    }
    let mut right = Vec::new();
    for j in col + 1..grid[row].len() {
        right.push(grid[row][j])
    }
    (left, right)
}

fn get_view_distance(height: u32, ray: &[u32]) -> u64 {
    let mut seen_trees = 0;
    for other_height in ray {
        seen_trees += 1;
        if other_height >= &height {
            break;
        }
    }
    seen_trees
}

fn problem1(lines: Vec<String>) -> u64 {
    let grid = parse_grid(&lines);
    let mut marks: HashSet<(usize, usize)> = HashSet::new();
    for row_idx in 0..lines.len() {
        let mut row = get_row(&grid, row_idx);
        add_visible(&row, &mut marks);
        row.reverse();
        add_visible(&row, &mut marks);
    }
    for col_idx in 0..lines[0].len() {
        let mut col = get_col(&grid, col_idx);
        add_visible(&col, &mut marks);
        col.reverse();
        add_visible(&col, &mut marks);
    }
    marks.len() as u64
}

fn problem2(lines: Vec<String>) -> u64 {
    let grid = parse_grid(&lines);
    let mut max_so_far: Option<u64> = None;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let row_rays = cast_row_ray(&grid, i, j);
            let col_rays = cast_col_ray(&grid, i, j);
            let height = grid[i][j];
            let result = get_view_distance(height, &row_rays.0)
                * get_view_distance(height, &row_rays.1)
                * get_view_distance(height, &col_rays.0)
                * get_view_distance(height, &col_rays.1);
            if let Some(m) = max_so_far {
                if m >= result {
                    continue;
                }
            }
            max_so_far = Some(result)
        }
    }
    max_so_far.unwrap()
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let lines: Vec<String> = br.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let result = match part {
        ProblemPart::P1 => problem1(lines),
        ProblemPart::P2 => problem2(lines),
    };
    println!("result: {}", result);
    Ok(())
}
