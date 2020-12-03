use crate::input;
use std::io;

#[derive(Debug)]
struct PassPolicy {
    min: u32,
    max: u32,
    glyph: char
}

#[derive(Debug)]
struct PassEntry<'a> {
    policy: PassPolicy,
    password: &'a str
}

impl<'a> PassEntry<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut split: Vec<&'a str> = input.split(": ").collect();
        assert_eq!(split.len(), 2);
        println!("{:?}", split);
        let policy_str = split[0];
        let password = split[1];
        split = policy_str.split(" ").collect();
        assert_eq!(split.len(), 2);
        assert_eq!(split[1].len(), 1);
        println!("{:?}", split);
        let glyph = split[1].chars().nth(0).unwrap();
        split = split[0].split("-").collect();
        println!("{:?}", split);
        assert_eq!(split.len(), 2);
        let policy = PassPolicy {
            min: split[0].parse::<u32>().unwrap(),
            max: split[1].parse::<u32>().unwrap(),
            glyph
        };
        PassEntry {
            policy,
            password
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut glyph_count: u32 = 0;
        for glyph in self.password.chars() {
            if glyph == self.policy.glyph {
                glyph_count += 1;
            }
        }
        if glyph_count < self.policy.min || glyph_count > self.policy.max {
            false
        } else {
            true
        }
    }
}

pub fn solve() -> Result<(), io::Error> {
    let input_lines = input::get_lines_input("day2")
        .expect("couldn't open input file inputs/day2");
    let mut count: u32 = 0;
    for line in input_lines {
        let entry = PassEntry::new(&line);
        if entry.is_valid() { count += 1; }
    }
    println!("valid passwords: {}", count);
    Ok(())
}