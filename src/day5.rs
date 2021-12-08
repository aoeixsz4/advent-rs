use std::io;
use crate::input;

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Clone)]
struct LineVec {
    start: Point,
    end: Point
}

fn part1(data: &[String]) -> i64 {
    let line_vectors = data.iter().map(|line|{
        let (start, end) = line.split_once(" -> ").unwrap();
        let (start_x, start_y) = start.split_once(",").unwrap();
        let (end_x, end_y) = end.split_once(",").unwrap();
        LineVec {
            start: Point { x: start_x.parse::<i64>().unwrap(), y: start_y.parse::<i64>().unwrap() },
            end: Point { x: end_x.parse::<i64>().unwrap(), y: end_y.parse::<i64>().unwrap() }
        }
    }).collect::<Vec<LineVec>>();
    let (mut max_x, mut max_y) = (0, 0);
    for line_vec in line_vectors.clone() {
        if line_vec.start.x > max_x { max_x = line_vec.start.x; }
        if line_vec.end.x > max_x { max_x = line_vec.end.x; }
        if line_vec.start.y > max_y { max_y = line_vec.start.y; }
        if line_vec.end.y > max_y { max_y = line_vec.end.y; }
    }
    let mut grid = Vec::new();
    for _j in 0 .. max_y + 1 {
        let mut row = Vec::new();
        for _i in 0 .. max_x + 1 {
            row.push(0);
        }
        grid.push(row);
    }
    for line_vec in line_vectors {
        if line_vec.start.x != line_vec.end.x && line_vec.start.y != line_vec.end.y {
            continue;
        }
        let mut cursor = Point { x: line_vec.start.x, y: line_vec.start.y };
        while cursor.x != line_vec.end.x || cursor.y != line_vec.end.y {
            grid[cursor.y as usize][cursor.x as usize] += 1;
            if cursor.y != line_vec.end.y {
                cursor.y += (line_vec.end.y - cursor.y)/(line_vec.end.y - cursor.y).abs();
            }
            if cursor.x != line_vec.end.x {
                cursor.x += (line_vec.end.x - cursor.x)/(line_vec.end.x - cursor.x).abs();
            }
        }
        grid[cursor.y as usize][cursor.x as usize] += 1;
    }
    let mut count = 0;
    for j in 0 .. max_y + 1 {
        for i in 0 .. max_x + 1 {
            if grid[j as usize][i as usize] > 1 { count += 1; }
        }
    }
    count
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day5")
        .expect("couldn't open input file for day5 (should be inputs/day5)");
    println!("part1: {}", part1(&data));
    Ok(())
}