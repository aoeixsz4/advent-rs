use std::io;
use std::collections::{HashMap, HashSet};
use crate::input;

#[derive(Debug)]
enum CaveType {
    Start,
    Small(String),
    Big(String),
    End
}

fn is_upper(name: &str) -> bool {
    name.chars().filter(|c| c.is_ascii_uppercase()).count() == name.len()
}

fn is_lower(name: &str) -> bool {
    name.chars().filter(|c| c.is_ascii_lowercase()).count() == name.len()
}

fn cave_type_from_name(name: &str) -> CaveType {
    if name.eq("start") {
        CaveType::Start
    } else if name.eq("end") {
        CaveType::End
    } else if is_lower(name) {
        CaveType::Small(name.to_string())
    } else if is_upper(name) {
        CaveType::Big(name.to_string())
    } else {
        unreachable!()
    }
}

#[derive(Debug)]
struct Cave {
    node_type: CaveType,
    links: HashSet<String>,
    visited: bool,
}

impl Cave {
    pub fn new(name: &str) -> Self {
        Cave { node_type: cave_type_from_name(name), links: HashSet::new(), visited: false }
    }

    pub fn link(&mut self, link: &str) -> bool {
        self.links.insert(link.to_string())
    }
}

type CaveSystem = HashMap<String, Cave>;

fn get_or_create_cave<'a> (system: &'a mut CaveSystem, name: &str) -> Cave {
    if let Some(cave) = system.remove(name) {
        cave
    } else {
        Cave::new(name)
    }
}

fn generate_cave_system(data: &[String]) -> CaveSystem {
    let mut system = CaveSystem::new();
    for (name_a, name_b) in data.iter().map(|l| l.split_once('-').unwrap()) {
        let mut cave_a = get_or_create_cave(&mut system, name_a);
        let mut cave_b = get_or_create_cave(&mut system, name_b);
        cave_a.link(name_b);
        cave_b.link(name_a);
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
    match cave_type_from_name(&name) {
        CaveType::Start => false,
        CaveType::Big(_) => true,
        CaveType::Small(name) => is_small_cave_allowed(&name, path, double_visit),
        CaveType::End => true,
    }
}

fn get_children(system: &mut CaveSystem, path: &Vec<String>, name: &str, double_visit: bool) -> Vec<(String, Vec<String>)> {
    let mut children = Vec::new();
    let cave = system.get(&name.to_string()).unwrap();
    if let CaveType::End = cave.node_type { return children; }
    for child_name in cave.links.clone() {
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
        let example_1_a_links = HashSet::from(["c", "start", "end", "b"].map(|s|s.to_string()));
        let system_1 = generate_cave_system(&example_1);
        assert_eq!(system_1.len(), 6);
        assert_eq!(system_1.get("A").unwrap().links, example_1_a_links);

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
        let example_2_kj_links = HashSet::from(["start", "sa", "HN", "dc"].map(|s|s.to_string()));
        let system_2 = generate_cave_system(&example_2);
        assert_eq!(system_2.len(), 7);
        assert_eq!(system_2.get("kj").unwrap().links, example_2_kj_links);
        assert_eq!(part1(&example_1), 10);
        assert_eq!(part1(&example_2), 19);

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
        assert_eq!(part1(&example_3), 226);
        assert_eq!(part2(&example_1), 36);
        assert_eq!(part2(&example_2), 103);
        assert_eq!(part2(&example_3), 3509);
    }
}