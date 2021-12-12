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
        ].into_iter().map(|s|s.to_string()).collect::<Vec<String>>();
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
        ].into_iter().map(|s|s.to_string()).collect::<Vec<String>>();;
        let example_2_kj_links = HashSet::from(["start", "sa", "HN", "dc"].map(|s|s.to_string()));
        let system_2 = generate_cave_system(&example_2);
        assert_eq!(system_2.len(), 7);
        assert_eq!(system_2.get("kj").unwrap().links, example_2_kj_links);
    }
}