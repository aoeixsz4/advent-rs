const PLR1_START: i64 = 5;
const PLR2_START: i64 = 9;

const ROLLS: [[i64; 3]; 27] = [
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 3],
    [1, 2, 1],
    [1, 2, 2],
    [1, 2, 3],
    [1, 3, 1],
    [1, 3, 2],
    [1, 3, 3],
    [2, 1, 1],
    [2, 1, 2],
    [2, 1, 3],
    [2, 2, 1],
    [2, 2, 2],
    [2, 2, 3],
    [2, 3, 1],
    [2, 3, 2],
    [2, 3, 3],
    [3, 1, 1],
    [3, 1, 2],
    [3, 1, 3],
    [3, 2, 1],
    [3, 2, 2],
    [3, 2, 3],
    [3, 3, 1],
    [3, 3, 2],
    [3, 3, 3],
];

fn dirac(plr1_pos: i64, plr2_pos: i64, plr1_score: i64, plr2_score: i64) -> (i64, i64) {
    let (mut plr1_wins, mut plr2_wins) = (0, 0);
    for roll in ROLLS {
        let plr1_pos_tmp = (plr1_pos + roll[0] + roll[1] + roll[2]) % 10;
        if plr1_score + plr1_pos_tmp >= 21 {
            plr1_wins += 1;
            continue;
        }

        for roll2 in ROLLS {
            let plr2_pos_tmp = (plr2_pos + roll2[0] + roll2[1] + roll2[2]) % 10;
            if plr2_score + plr2_pos_tmp >= 21 {
                plr2_wins += 1;
                continue;
            }

            let (wins1, wins2) = dirac(
                plr1_pos_tmp,
                plr2_pos_tmp,
                plr1_score + plr1_pos_tmp,
                plr2_score + plr2_pos_tmp,
            );
            plr1_wins += wins1;
            plr2_wins += wins2;
        }
    }
    (plr1_wins, plr2_wins)
}

fn part2() -> i64 {
    let (plr1_wins, plr2_wins) = dirac(PLR1_START, PLR2_START, 0, 0);
    plr1_wins.max(plr2_wins)
}

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
    println!("part2: {}", part2());
}
