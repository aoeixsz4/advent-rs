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
    } else if is_lower(name) {
        CaveType::Small(name.to_string())
    } else if is_upper(name) {
        CaveType::Big(name.to_string())
    } else if name.eq("end") {
        CaveType::End
    } else {
        unreachable!()
    }
}

fn cave_type_to_name<'a> (cave_type: &'a CaveType) -> &'a str {
    match &cave_type {
        &CaveType::Start => "start",
        &CaveType::Small(s) => &s,
        &CaveType::Big(s) => &s,
        &CaveType::End => "end"
    }
}

#[derive(Debug)]
struct Cave {
    node_type: CaveType,
    links: HashSet<String>
}

impl Cave {
    pub fn new(name: &str) -> Self {
        Cave { node_type: cave_type_from_name(name), links: HashSet::new() }
    }

    pub fn new_with_link(name: &str, link: &str) -> Self {
        let mut links = HashSet::new();
        links.insert(link.to_string());
        Cave { node_type: cave_type_from_name(name), links }
    }
}

type CaveSystem = HashMap<String, Cave>;

fn insert_cave(system: &mut CaveSystem, mut cave: Cave) {
    let name = cave_type_to_name(&cave.node_type);
    if let Some(mut existing_cave) = system.get(name) {
        let mut links = existing_cave.links.clone();
        for link in &cave.links {
            links.insert(link.to_string());
        }
        cave.links = links;
    }
    system.insert(name.to_string(), cave);
}

fn generate_cave_system(data: &[&str]) -> CaveSystem {
    let mut system = CaveSystem::new();
    data.iter().map(|l| l.split_once('-').unwrap())
        .map(|(a, b)| {
            (Cave::new_with_link(a, b), Cave::new_with_link(b, a))
        }).for_each(|(cave_a, cave_b)| {
            insert_cave(&mut system, cave_a);
            insert_cave(&mut system, cave_b);
        });
    system
}

fn part1(data: &[String]) -> i64 {
    0
}

fn part2(data: &[String]) -> i64 {
    0
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day12")
        .expect("couldn't open input file for day11 (should be inputs/day12)");
    //println!("part1: {}", part1(&data));
    //println!("part2: {}", part2(&data));
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
        ];
        let foo = generate_cave_system(&example_1);
        println!("{:?}", foo);
    }
}