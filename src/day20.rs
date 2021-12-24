use std::{iter::FromIterator, time::Instant};
const INPUT: &str = include_str!("day20.txt");
const GRID_SIZE: usize = 300;
const RULE_SIZE: usize = 600;

fn get_ruleset(r: &str) -> Vec<u16> {
    r.chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => unreachable!(),
        })
        .collect()
}

fn get_initial_grid(g: &str) -> Vec<Vec<u16>> {
    let mut grid: Vec<Vec<u16>> = Vec::new();
    (0..GRID_SIZE).for_each(|_| grid.push((0..GRID_SIZE).map(|_| 0).collect::<Vec<u16>>()));
    let (mut j, mut i) = (55, 55);
    g.lines().for_each(|l| {
        l.chars().for_each(|c| match c {
            '#' => {
                grid[j][i] = 1;
                i += 1;
            }
            '.' => {
                grid[j][i] = 0;
                i += 1;
            }
            _ => unreachable!(),
        });
        j += 1;
        i = 55;
    });
    grid
}

fn get_rule(g: &[Vec<u16>], r: &[u16], j: usize, i: usize) -> u16 {
    let mut val: usize = 0;

    val |= (g[j + 1][i + 1] | (g[j + 1][i] << 1) | (g[j + 1][i - 1] << 2)) as usize;
    val |= ((g[j][i + 1] << 3) | (g[j][i] << 4) | (g[j][i - 1] << 5)) as usize;
    val |= ((g[j - 1][i + 1] << 6) | (g[j - 1][i] << 7) | (g[j - 1][i - 1] << 8)) as usize;
    //if val != 0 {
    //    println!("index {}", val);
    //}
    r[val]
}

fn transform(g: &[Vec<u16>], r: &[u16]) -> Vec<Vec<u16>> {
    let mut grid: Vec<Vec<u16>> = Vec::new();
    (0..GRID_SIZE).for_each(|_| grid.push((0..GRID_SIZE).map(|_| 1).collect::<Vec<u16>>()));

    for j in 1..GRID_SIZE - 1 {
        for i in 1..GRID_SIZE - 1 {
            grid[j][i] = get_rule(g, r, j, i);
        }
    }
    grid
}

fn transform2(g: &[Vec<u16>], r: &[u16]) -> Vec<Vec<u16>> {
    let mut grid: Vec<Vec<u16>> = Vec::new();
    (0..GRID_SIZE).for_each(|_| grid.push((0..GRID_SIZE).map(|_| 0).collect::<Vec<u16>>()));

    for j in 1..GRID_SIZE - 1 {
        for i in 1..GRID_SIZE - 1 {
            grid[j][i] = get_rule(g, r, j, i);
        }
    }
    grid
}

fn part1(s: &str) -> usize {
    let (rules, map) = s.split_once("\n\n").unwrap();
    let ruleset = get_ruleset(rules);
    let map = get_initial_grid(map);
    let new_map = transform(&map, &ruleset);
    let second_iter = transform2(&new_map, &ruleset);
    //print_grid(&new_map);
    second_iter
        .iter()
        .map(|line| line.iter().filter(|x| **x != 0).count())
        .sum()
}

fn part2(s: &str) -> usize {
    let (rules, map) = s.split_once("\n\n").unwrap();
    let ruleset = get_ruleset(rules);
    let map = get_initial_grid(map);
    let mut new_map = transform(&map, &ruleset);
    let mut second_iter = transform2(&new_map, &ruleset);
    for _ in 1..25 {
        new_map = transform(&second_iter, &ruleset);
        second_iter = transform2(&new_map, &ruleset);
    }
    //print_grid(&new_map);
    second_iter
        .iter()
        .map(|line| line.iter().filter(|x| **x != 0).count())
        .sum()
}

pub fn solve() {
    let t0 = Instant::now();
    let pt1 = part1(INPUT);
    println!("part1: {}", pt1);
    let t1 = t0.elapsed();
    println!("part1 time: {:?}", t1);
    let pt2 = part2(INPUT);
    println!("part2: {}", pt2);
    let t1 = t0.elapsed();
    println!("part2 time: {:?}", t1);
}

fn print_grid(g: &[Vec<u16>]) {
    g.iter().for_each(|line| {
        println!(
            "{}",
            &line
                .iter()
                .map(|x| {
                    match *x {
                        0 => '.',
                        1 => '#',
                        _ => unreachable!(),
                    }
                })
                .collect::<String>()
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = include_str!("day20-ex1.txt");

    #[test]
    fn test() {
        const TIMES: u32 = 1;
        let t0 = Instant::now();
        let (rules, map) = EX1.split_once("\n\n").unwrap();
        let ruleset = get_ruleset(rules);
        let map = get_initial_grid(map);
        let new_map = transform(&map, &ruleset);
        let second_iter = transform2(&new_map, &ruleset);
        let count: usize = second_iter
            .iter()
            .map(|line| line.iter().filter(|x| **x != 0).count())
            .sum();
        print_grid(&map);
        assert_eq!(count, 35);
        let t1 = t0.elapsed();
        println!("duration: {:?}", t1 / TIMES);
    }
}
