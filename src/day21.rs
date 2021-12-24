const PLR1_START: i64 = 5;
const PLR2_START: i64 = 9;

fn part1() -> i64 {
    let mut die = 1;
    let mut rolls = 0;
    let mut plr1 = PLR1_START;
    let mut plr2 = PLR2_START;
    let mut plr1_score = 0;
    let mut plr2_score = 0;
    loop {
        plr1 = (plr1 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr1 = (plr1 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr1 = (plr1 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr1_score += plr1;
        if plr1_score >= 1000 {
            return plr2_score * rolls;
        }
        plr2 = (plr2 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr2 = (plr2 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr2 = (plr2 + die) % 10;
        die += 1;
        rolls += 1;
        if die == 101 {
            die = 1;
        }
        plr2_score += plr2;
        if plr2_score >= 1000 {
            return plr1_score * rolls;
        }
    }
}

pub fn solve() {
    println!("part1: {}", part1());
}
