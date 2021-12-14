use std::collections::HashMap;
use std::iter::FromIterator;
const INPUT: &str = include_str!("day14.txt");

fn get_pairs<'a> (s: &'a str) -> Vec<&'a str> {
    (0..(s.len()-1)).map(|i|&s[i..i+2]).collect::<Vec<&'a str>>()
}

fn recurse(
    counts: HashMap<String, u64>,
    rules: HashMap<String, Vec<String>>,
    times: usize,
    single_counts: &mut HashMap<char, u64>,
    recursion_count: &mut usize,
) -> HashMap<String, u64> {
    if *recursion_count == times {
        return counts;
    }
    *recursion_count += 1;
    let mut new_counts: HashMap<String, u64> = HashMap::new();
    for (key, count) in counts {
        let rule = rules.get(&key).unwrap();
        assert_eq!(rule.len(), 2);
        *new_counts.entry(rule[0].clone()).or_default() += count;
        *new_counts.entry(rule[1].clone()).or_default() += count;
        *single_counts.entry(rule[1].chars().nth(0).unwrap()).or_default() += count;
    }
    recurse(new_counts, rules, times, single_counts, recursion_count)
}

fn run(primer: &str, s: &str, times: usize) -> u64 {
    let rules: HashMap<String, Vec<String>> = HashMap::from_iter(s.lines().map(|l|{
        let (left, right) = l.split_once(" -> ").unwrap();
        assert_eq!(left.len(), 2);
        assert_eq!(right.len(), 1);
        let a = String::from_iter([left.chars().nth(0).unwrap(), right.chars().nth(0).unwrap()].iter());
        let b = String::from_iter([right.chars().nth(0).unwrap(), left.chars().nth(1).unwrap()].iter());
        (left.to_string(), vec![a, b])
    }).into_iter());
    let mut single_counts: HashMap<char, u64> = HashMap::new();
    for c in primer.chars() {
        *single_counts.entry(c).or_default() += 1;
    }
    let init_pairs = get_pairs(primer);
    let counts: HashMap<String, u64> = HashMap::from_iter(rules.keys().map(|k|{
        (k.clone(), init_pairs.iter().filter(|pair|pair.to_string() == *k).count() as u64)
    }).into_iter());
    let mut recursion_count = 0;
    recurse(counts,  rules, times, &mut single_counts, &mut recursion_count);
    single_counts.values().max().unwrap() - single_counts.values().min().unwrap()
}

pub fn solve() {
    let (primer, s) = INPUT.split_once("\n\n").unwrap();
    println!("part1: {}", run(primer, s, 10));
    println!("part2: {}", run(primer, s, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const EXAMPLE1: &str = include_str!("day14-ex1.txt");
        let (primer, s) = EXAMPLE1.split_once("\n\n").unwrap();
        assert_eq!(run(primer, s, 10), 1588);
        assert_eq!(run(primer, s, 40), 2188189693529);
    }
}