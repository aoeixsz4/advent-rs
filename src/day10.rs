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

fn get_completion_string(line: &String) -> Option<String> {
    let mut open_token_stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => open_token_stack.push('('),
            '[' => open_token_stack.push('['),
            '{' => open_token_stack.push('{'),
            '<' => open_token_stack.push('<'),
            ')' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '(' { return None; }
            },
            ']' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '[' { return None; }
            },
            '}' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '{' { return None; }
            },
            '>' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '<' { return None; }
            },
            _ => unreachable!()
        }
    }
    Some(
        open_token_stack.iter().rev()
        .map(|c|{
            match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!()
            }
        }).collect::<String>()
    )
}

fn score_completion_string(comp: &String) -> i64 {
    comp.chars().map(|c| match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!()
    }).fold(0, |acc, x| {
        acc * 5 + x
    })
}

fn part1(data: &[String]) -> i64 {
    data.iter().map(|l| score_line(l)).sum()
}

fn part2(data: &[String]) -> i64 {
    let mut scores = data.iter().filter_map(|l|
        get_completion_string(l)
    ).map(|completion|
        score_completion_string(&completion)
    ).collect::<Vec<i64>>();
    scores.sort();
    let len = scores.len();
    if len % 2 != 1 { panic!("number of scores isn't odd!"); }
    scores[(len-1)/2]
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day10")
        .expect("couldn't open input file for day10 (should be inputs/day10)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
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
        assert_eq!(part2(&input), 288957);
    }
}