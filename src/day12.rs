use std::io;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
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

struct Tree {
    path: Vec<String>,
    children: Mutex<Vec<Arc<Tree>>>,
    cave_name: String,
}

impl Tree {
    pub fn init() -> Arc<Self> {
        Arc::new(Tree {
            path: vec!["start".to_string()],
            children: Mutex::new(Vec::new()),
            cave_name: "start".to_string()
        })
    }

    pub fn add_children(self: Arc<Self>, system: &mut CaveSystem) -> Vec<Arc<Tree>> {
        let mut children = Vec::new();
        let cave = system.get_mut(&self.cave_name).unwrap();
        if let CaveType::End = cave.node_type { return children; }
        cave.visited = true;
        for child_name in cave.links.clone() {
            match cave_type_from_name(&child_name) {
                CaveType::Small(_) if self.path.contains(&child_name) => continue,
                CaveType::Start => continue,
                _ => ()
            }
            let mut new_path = self.path.clone();
            new_path.push(child_name.clone());
            let child = Arc::new(Tree { path: new_path, children: Mutex::new(Vec::new()), cave_name: child_name.clone() });
            children.push(Arc::clone(&child));
            let mut mutex_lock = self.children.lock().unwrap();
            mutex_lock.push(child);
        }
        children
    }
}

fn part1(data: &[String]) -> usize {
    let mut system = generate_cave_system(data);
    let tree_root = Tree::init();
    let mut layer_nodes = Vec::new();
    let mut complete_paths = Vec::new();
    layer_nodes.push(tree_root);
    while layer_nodes.len() > 0 {
        let mut next_layer = Vec::new();
        for node in layer_nodes {
            let mut children = node.add_children(&mut system);
            for child in children.clone().iter() {
                if child.cave_name == "end" {
                    complete_paths.push(child.path.join(","));
                }
            }
            next_layer.append(&mut children);
        }
        layer_nodes = next_layer;
    }
    complete_paths.len()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day12")
        .expect("couldn't open input file for day11 (should be inputs/day12)");
    println!("part1: {}", part1(&data));
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
    }
}