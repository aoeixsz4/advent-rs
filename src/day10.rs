use std::io;
use crate::input;

fn score_line(line: &String) -> i64 {
    let mut open_token_stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => open_token_stack.push('('),
            '[' => open_token_stack.push('['),
            '{' => open_token_stack.push('{'),
            '<' => open_token_stack.push('<'),
            ')' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '(' { return 3; }
            },
            ']' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '[' { return 57; }
            },
            '}' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '{' { return 1197; }
            },
            '>' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '<' { return 25137; }
            },
            _ => unreachable!()
        }
    }
    0
}

fn part1(data: &[String]) -> i64 {
    data.iter().map(|l| score_line(l)).sum()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day10")
        .expect("couldn't open input file for day10 (should be inputs/day10)");
    println!("part1: {}", part1(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];
        assert_eq!(part1(&input), 26397);
    }
}