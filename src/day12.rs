use std::io;
use std::collections::{HashMap, HashSet};
use crate::input;

type Cave = HashSet<String>;
type CaveSystem = HashMap<String, Cave>;

fn is_lower(name: &str) -> bool {
    name.as_bytes().iter().all(u8::is_ascii_lowercase)
}

fn generate_cave_system(data: &[String]) -> CaveSystem {
    let mut system = CaveSystem::new();
    for (name_a, name_b) in data.iter().map(|l| l.split_once('-').unwrap()) {
        system.entry(name_a.to_string()).or_default().insert(name_b.to_string());
        system.entry(name_b.to_string()).or_default().insert(name_a.to_string());
    }
    system
}

fn visit(system: &CaveSystem, node: &str, visited: &mut HashSet<String>, smol: &mut String) -> usize {
    if node == "end" { return 1; }
    if is_lower(node) {
        if visited.contains(node) {
            if smol.len() != 0 {
                return 0;
            } else {
                *smol = node.to_string();
            }
        } else {
            visited.insert(node.to_string());
        }
    }
    let count = system.get(node).unwrap().iter().filter(|dest|dest.as_str() != "start").map(|dest|{
        visit(system, dest, visited, smol)
    }).sum();
    if is_lower(node) {
        if smol.as_str() != node {
            visited.remove(&node.to_string());
        } else {
            *smol = String::from("")
        }
    }
    count
}

pub fn part1(data: &[String]) -> usize {
    let paths = generate_cave_system(data);
    let mut visited = HashSet::new();
    let mut smol = String::from("foo");
    visit(&paths, "start", &mut visited, &mut smol)
}

pub fn part2(data: &[String]) -> usize {
    let paths = generate_cave_system(data);
    let mut visited = HashSet::new();
    let mut smol = String::from("");
    let foo = visit(&paths, "start", &mut visited, &mut smol);
    foo
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day12")
        .expect("couldn't open input file for day12 (should be inputs/day12)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example_1 = [
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].iter().map(|s|s.to_string()).collect::<Vec<String>>();

        let example_2 = [
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ].iter().map(|s|s.to_string()).collect::<Vec<String>>();

        let example_3 = [
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW",
        ].iter().map(|s|s.to_string()).collect::<Vec<String>>();
        assert_eq!(part1(&example_1), 10);
        assert_eq!(part1(&example_2), 19);
        assert_eq!(part1(&example_3), 226);
        assert_eq!(part2(&example_1), 36);
        assert_eq!(part2(&example_2), 103);
        assert_eq!(part2(&example_3), 3509);
    }
}