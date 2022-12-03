use std::io;

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    pub fn score(&self) -> i32 {
        use Result::*;
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

enum Play {
    Rock,
    Scissors,
    Paper,
}

impl Play {
    pub fn from_char(c: char) -> Self {
        use Play::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("wut")
        }
    }

    pub fn score(&self) -> i32 {
        use Play::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    pub fn result(&self, other: Play) -> i32 {
        use Play::*;
        use Result::*;
        self.score() + match self {
            Rock => {
                match other {
                    Rock => Draw,
                    Scissors => Win,
                    Paper => Lose,
                }
            },
            Scissors => {
                match other {
                    Rock => Lose,
                    Scissors => Draw,
                    Paper => Win,
                }
            },
            Paper => {
                match other {
                    Rock => Win,
                    Scissors => Lose,
                    Paper => Draw,
                }
            },
        }.score()
    }
}

fn main() {
    let lines = io::stdin().lines();
    let mut score = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        let real_line = unwrapped.as_bytes();
        let them = Play::from_char(real_line[0] as char);
        let me = Play::from_char(real_line[2] as char);
        score += me.result(them);
    }

    println!("{}", score);
}
