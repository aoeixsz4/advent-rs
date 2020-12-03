use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub fn get_numeric_input(day: &str) -> Result<Vec<u32>, io::Error> {
    let mut output = Vec::new();
    let file_handle = File::open(format!("inputs/{}", day))?;
    let reader = BufReader::new(file_handle);
    for try_line in reader.lines() {
        match try_line {
            Ok(line) => output.push(line.parse::<u32>().expect("invalid input format")),
            Err(e) => return Err(e)
        }
    }
    Ok(output)
}

pub fn get_lines_input(day: &str) -> Result<Vec<String>, io::Error> {
    let mut output = Vec::new();
    let file_handle = File::open(format!("inputs/{}", day))?;
    let reader = BufReader::new(file_handle);
    for try_line in reader.lines() {
        match try_line {
            Ok(line) => output.push(line),
            Err(e) => return Err(e)
        }
    }
    Ok(output)
}