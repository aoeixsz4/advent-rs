use std::io;
use std::collections::{HashMap, HashSet};
use crate::input;

type Cave = HashSet<String>;
type CaveSystem = HashMap<String, Cave>;

fn is_lower(name: &str) -> bool {
    name.chars().filter(|c| c.is_ascii_lowercase()).count() == name.len()
}

fn get_or_create_cave (system: &mut CaveSystem, name: &str) -> Cave {
    if let Some(cave) = system.remove(name) {
        cave
    } else {
        Cave::new()
    }
}

fn generate_cave_system(data: &[String]) -> CaveSystem {
    let mut system = CaveSystem::new();
    for (name_a, name_b) in data.iter().map(|l| l.split_once('-').unwrap()) {
        let mut cave_a = get_or_create_cave(&mut system, name_a);
        let mut cave_b = get_or_create_cave(&mut system, name_b);
        cave_a.insert(name_b.to_string());
        cave_b.insert(name_a.to_string());
        system.insert(name_a.to_string(), cave_a);
        system.insert(name_b.to_string(), cave_b);
    }
    system
}

fn is_small_cave_allowed(name: &str, path: &Vec<String>, double_visit: bool) -> bool {
    if !path.contains(&name.to_string()) { return true; }
    if !double_visit { return false; }
    let mut cave_counts = HashSet::new();
    for name in path.clone().into_iter().filter(|s|s.as_str() != "start" && s.as_str() != "end" && is_lower(&s)) {
        if cave_counts.contains(&name) {
            return false;
        } else {
            cave_counts.insert(name.clone());
        }
    }
    true
}

fn is_cave_allowed(name: &str, path: &Vec<String>, double_visit: bool) -> bool {
    if name == "start" || is_lower(name) && !is_small_cave_allowed(name, path, double_visit) {
        false
    } else {
        true
    }
}

fn get_children(system: &mut CaveSystem, path: &Vec<String>, name: &str, double_visit: bool) -> Vec<(String, Vec<String>)> {
    let mut children = Vec::new();
    if name == "end" { return children; }
    let links = system.get(&name.to_string()).unwrap();
    for child_name in links {
        if !is_cave_allowed(&child_name, path, double_visit) { continue; }
        let mut new_path = path.clone();
        new_path.push(child_name.clone());
        children.push((child_name.clone(), new_path));
    }
    children
}

fn common(data: &[String], double_visit: bool) -> Vec<Vec<String>> {
    let mut system = generate_cave_system(data);
    let mut layer_nodes = Vec::new();
    let mut complete_paths = Vec::new();
    layer_nodes.push(("start".to_string(), vec!["start".to_string()]));
    while layer_nodes.len() > 0 {
        let mut next_layer = Vec::new();
        for (name, path) in layer_nodes {
            if name == "end" {
                complete_paths.push(path);
            } else {
                let mut children = get_children(&mut system, &path, &name, double_visit);
                next_layer.append(&mut children);
            }
        }
        layer_nodes = next_layer;
    }
    complete_paths
}

pub fn part1(data: &[String]) -> usize {
    common(data, false).len()
}

pub fn part2(data: &[String]) -> usize {
    common(data, true).len()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day12")
        .expect("couldn't open input file for day11 (should be inputs/day12)");
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