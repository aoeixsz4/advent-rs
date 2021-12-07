mod day1;
mod day2;
mod day3;
mod day4;
//mod day5;
mod day6;
mod day7;
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
            "3" | "day3" => { println!("\t\tDAY THREE"); day3::solve()? },
            "4" | "day4" => { println!("\t\tDAY FOUR"); day4::solve()? },
            //"5" | "day5" => { println!("\t\tDAY FIVE"); day5::solve()? },
            "6" | "day6" => { println!("\t\tDAY SIX"); day6::solve()? },
            "7" | "day7" => { println!("\t\tDAY SEVEN"); day7::solve()? },
            _ => writeln!(io::stderr(), "unknown argument: {}", &day)?
        }
        println!("");
    }
    Ok(())
}