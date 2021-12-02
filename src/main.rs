mod day1;
mod day2;
//mod day3;
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
            "1" | "day1" => { println!("\t\tDAY ONE"); day1::solve()? },
            "2" | "day2" => { println!("\t\tDAY TWO"); day2::solve()? },
            //"3" | "day3" => { println!("\t\tDAY TWO"); day3::solve()? },
            _ => writeln!(io::stderr(), "unknown argument: {}", &day)?
        }
        println!("");
    }
    Ok(())
}