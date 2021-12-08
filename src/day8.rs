use std::io;
use crate::input;

fn count_identifiable_digits(output_digits: &Vec<Vec<&str>>) -> i64 {
    output_digits.iter().map(|entry|{
        entry.iter().filter(|s|s.len()==2||s.len()==3||s.len()==4||s.len()==7).count() as i64
    }).sum()
}

fn part1(data: &[String]) -> i64 {
    let mut wire_combinations = Vec::new();
    let mut output_digits = Vec::new();
    for entry in data {
        let (combinations_string, outputs_string) = entry.split_once("|").unwrap();
        wire_combinations.push(combinations_string.split_whitespace().collect::<Vec<&str>>());
        output_digits.push(outputs_string.split_whitespace().collect::<Vec<&str>>());
    }
    count_identifiable_digits(&output_digits)
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day8")
        .expect("couldn't open input file for day8 (should be inputs/day8)");
    println!("part1: {}", part1(&data));
    Ok(())
}