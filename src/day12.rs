use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("day12.txt");

fn is_lower(name: &str) -> bool {
    name.as_bytes().iter().all(u8::is_ascii_lowercase)
}

fn visit<'a>(
    system: &HashMap<&str, HashSet<&'a str>>,
    node: &'a str,
    visited: &mut HashSet<&'a str>,
    smol: &mut &'a str,
) -> usize {
    if node == "end" {
        return 1;
    }
    if is_lower(node) {
        if visited.contains(node) {
            if !smol.is_empty() {
                return 0;
            } else {
                *smol = node;
            }
        } else {
            visited.insert(node);
        }
    }
    let count = system
        .get(node)
        .unwrap()
        .iter()
        .filter(|dest| **dest != "start")
        .map(|dest| visit(system, dest, visited, smol))
        .sum();
    if is_lower(node) {
        if *smol != node {
            visited.remove(&node);
        } else {
            *smol = ""
        }
    }
    count
}

fn run(data: &str, pt2: bool) -> usize {
    let mut system: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in data.lines().map(|l| l.split_once('-').unwrap()) {
        system.entry(a).or_default().insert(b);
        system.entry(b).or_default().insert(a);
    }
    let mut visited = HashSet::new();
    let mut smol = if pt2 { "" } else { "foo" };
    visit(&system, "start", &mut visited, &mut smol)
}

fn part1(data: &str) -> usize {
    run(data, false)
}

fn part2(data: &str) -> usize {
    run(data, true)
}

pub fn solve() {
    const TIMES: u32 = 10;

    let t0 = Instant::now();
    let (mut p1, mut p2) = (0, 0);
    for _ in 0..TIMES {
        p1 = part1(INPUT);
        p2 = part2(INPUT);
    }
    let t1 = t0.elapsed();
    eprintln!("{:?}", t1 / TIMES);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const EXAMPLE1: &str = include_str!("day12-ex1.txt");
        const EXAMPLE2: &str = include_str!("day12-ex2.txt");
        const EXAMPLE3: &str = include_str!("day12-ex3.txt");
        assert_eq!(part1(EXAMPLE1), 10);
        assert_eq!(part1(EXAMPLE2), 19);
        assert_eq!(part1(EXAMPLE3), 226);
        assert_eq!(part2(EXAMPLE1), 36);
        assert_eq!(part2(EXAMPLE2), 103);
        assert_eq!(part2(EXAMPLE3), 3509);
    }
}
