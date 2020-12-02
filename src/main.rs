mod day1;
mod input;

use chrono::{Datelike, Utc};
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut days: Vec<String> = env::args().collect();
    let _discard = days.remove(0);
    if days.len() == 0 { days.push(Utc::now().day().to_string()); }
    for day in days {
        match day.as_str() {
            "1" | "day1" => day1::solve(),
            "2" | "day2" => write!(io::stderr(), "sorry, day 2 not implemented :(\n")?,
            _ => write!(io::stderr(), "unknown argument: {}\n", &day)?
        }
    }
    Ok(())
}
