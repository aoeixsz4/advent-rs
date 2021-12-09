use std::{io, collections::HashMap};
use crate::input;

fn count_identifiable_digits(output_digits: &Vec<Vec<&str>>) -> i64 {
    output_digits.iter().map(|entry|{
        entry.iter().filter(|s|s.len()==2||s.len()==3||s.len()==4||s.len()==7).count() as i64
    }).sum()
}

fn fully_resolved(display: &Vec<Vec<char>>) -> bool {
    for segment in display {
        if segment.len() > 1 {
            return false;
        }
    }
    true
}

fn decode_combinations(wire_combinations: &Vec<&str>) -> Vec<char> {
    let mut display = Vec::new();
    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut found_frequencies: HashMap<char, i64> = HashMap::new();
    for _i in 0 .. chars.len() {
        display.push(chars.clone());
    }
    for c in &chars {
        found_frequencies.insert(*c, 0);
    }
    for combination in wire_combinations {
        for c in &chars {
            let freq = *found_frequencies.get(c).unwrap();
            if combination.contains(*c) {
                found_frequencies.insert(*c, freq + 1);
            }
        }
    }
    for c in &chars {
        let freq = found_frequencies.get(c).unwrap();
        if *freq == 6 {
            display[1] = vec![*c];
        }
        if *freq == 4 {
            display[4] = vec![*c];
        }
        if *freq == 9 {
            display[5] = vec![*c];
        }
    }
    for combination in wire_combinations {
        if combination.len() == 2 {
            display[2] = display[2].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[5] = display[5].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
        } else if combination.len() == 3 {
            display[0] = display[0].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[2] = display[2].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[5] = display[5].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
        } else if combination.len() == 4 {
            display[1] = display[1].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[2] = display[2].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[3] = display[3].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[5] = display[5].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
        } else if combination.len() == 7 {
            display[1] = display[1].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[2] = display[2].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[3] = display[3].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
            display[5] = display[5].clone().into_iter().filter(|c|combination.contains(*c)).collect::<Vec<char>>();
        }
    }
    while !fully_resolved(&display) {
        for i in 0 .. chars.len() {
            if display[i].len() == 1 {
                let taken = display[i][0];
                for j in 0 .. chars.len() {
                    if j != i {
                        display[j] = display[j].clone().into_iter().filter(|c|*c != taken).collect::<Vec<char>>();
                    }
                }
            }
        }
    }
    for segment in display.clone() {
        if segment.len() > 1 { panic!("one segment is still unambiguous: {:?}", segment); }
    }
    display.clone().into_iter().map(|d|d[0]).collect()
}

const ZERO: [bool; 7] = [true, true, true, false, true, true, true];
const ONE: [bool; 7] = [false, false, true, false, false, true, false];
const TWO: [bool; 7] = [true, false, true, true, true, false, true];
const THREE: [bool; 7] = [true, false, true, true, false, true, true];
const FOUR: [bool; 7] = [false, true, true, true, false, true, false];
const FIVE: [bool; 7] = [true, true, false, true, false, true, true];
const SIX: [bool; 7] = [true, true, false, true, true, true, true];
const SEVEN: [bool; 7] = [true, false, true, false, false, true, false];
const EIGHT: [bool; 7] = [true, true, true, true, true, true, true];
const NINE: [bool; 7] = [true, true, true, true, false, true, true];

fn resolve_digit(display: &Vec<char>, digit_string: &str) -> i64 {
    let mut lit_segments: [bool; 7] = [false; 7];
    for i in 0 .. display.len() {
        if digit_string.contains(display[i]) {
            lit_segments[i] = true;
        }
    }
    if lit_segments.eq(&ZERO) {
        0
    } else if lit_segments.eq(&ONE) {
        1
    } else if lit_segments.eq(&TWO) {
        2
    } else if lit_segments.eq(&THREE) {
        3
    } else if lit_segments.eq(&FOUR) {
        4
    } else if lit_segments.eq(&FIVE) {
        5
    } else if lit_segments.eq(&SIX) {
        6
    } else if lit_segments.eq(&SEVEN) {
        7
    } else if lit_segments.eq(&EIGHT) {
        8
    } else if lit_segments.eq(&NINE) {
        9
    } else {
        -1
    }
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

fn part2(data: &[String]) -> i64 {
    let mut sum = 0;
    for entry in data {
        let (combinations_string, outputs_string) = entry.split_once("|").unwrap();
        let wire_combinations= combinations_string.split_whitespace().collect::<Vec<&str>>();
        let decoded_display = decode_combinations(&wire_combinations);
        let output_digits = outputs_string.split_whitespace().collect::<Vec<&str>>();
        let number = resolve_digit(&decoded_display, output_digits[0]) * 1000
            + resolve_digit(&decoded_display, output_digits[1]) * 100
            + resolve_digit(&decoded_display, output_digits[2]) * 10
            + resolve_digit(&decoded_display, output_digits[3]);
        sum += number;
    }
    sum
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day8")
        .expect("couldn't open input file for day8 (should be inputs/day8)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}