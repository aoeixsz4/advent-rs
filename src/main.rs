mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use chrono::{Datelike, Utc};
use std::env;
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut days: Vec<String> = env::args().collect();
    let _discard = days.remove(0);
    if days.len() == 0 {
        days.push(Utc::now().day().to_string());
    }
    for day in days {
        match day.as_str() {
            "1" | "day1" => {
                println!("\t\tDAY ONE");
                day1::solve();
            }
            "2" | "day2" => {
                println!("\t\tDAY TWO");
                day2::solve();
            }
            "3" | "day3" => {
                println!("\t\tDAY THREE");
                day3::solve();
            }
            "4" | "day4" => {
                println!("\t\tDAY FOUR");
                day4::solve();
            }
            "5" | "day5" => {
                println!("\t\tDAY FIVE");
                day5::solve();
            }
            "6" | "day6" => {
                println!("\t\tDAY SIX");
                day6::solve();
            }
            "7" | "day7" => {
                println!("\t\tDAY SEVEN");
                day7::solve();
            }
            "8" | "day8" => {
                println!("\t\tDAY EIGHT");
                day8::solve();
            }
            "9" | "day9" => {
                println!("\t\tDAY NINE");
                day9::solve();
            }
            "10" | "day10" => {
                println!("\t\tDAY TEN");
                day10::solve();
            }
            "11" | "day11" => {
                println!("\t\tDAY ELEVEN");
                day11::solve();
            }
            "12" | "day12" => {
                println!("\t\tDAY TWELVE");
                day12::solve();
            }
            "13" | "day13" => {
                println!("\t\tDAY THIRTEEN");
                day13::solve();
            }
            "14" | "day14" => {
                println!("\t\tDAY FOURTEEN");
                day14::solve();
            }
            "15" | "day15" => {
                println!("\t\tDAY FIFTEEN");
                day15::solve();
            }
            "16" | "day16" => {
                println!("\t\tDAY SIXTEEN");
                day16::solve();
            }
            "17" | "day17" => {
                println!("\t\tDAY SEVENTEEN");
                day17::solve();
            }
            "18" | "day18" => {
                println!("\t\tDAY EIGHTEEN");
                day18::solve();
            }
            "19" | "day19" => {
                println!("\t\tDAY NINETEEN");
                day19::solve();
            }
            "20" | "day20" => {
                println!("\t\tDAY TWENTY");
                day20::solve();
            }
            "21" | "day21" => {
                println!("\t\tDAY TWENTY ONE");
                day21::solve();
            }
            _ => {
                writeln!(io::stderr(), "unknown argument: {}", &day)?;
            }
        }
    }
    Ok(())
}
