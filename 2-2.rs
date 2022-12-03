use std::io;

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    pub fn from_char(c: char) -> Self {
        use Result::*;
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("wut")
        }
    }

    pub fn score(&self) -> i32 {
        use Result::*;
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug)]
enum Play {
    Rock,
    Scissors,
    Paper,
}

impl Play {
    pub fn from_char(c: char) -> Self {
        use Play::*;
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
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

    pub fn play_for_result(&self, result: Result) -> Self {
        use Play::*;
        use Result::*;

        match self {
            Rock => {
                match result {
                    Win => Paper,
                    Lose => Scissors,
                    Draw => Rock,
                }
            },
            Scissors => {
                match result {
                    Win => Rock,
                    Lose => Paper,
                    Draw => Scissors,
                }
            },
            Paper => {
                match result {
                    Win => Scissors,
                    Lose => Rock,
                    Draw => Paper,
                }
            },
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
        let me = them.play_for_result(Result::from_char(real_line[2] as char));
        score += me.result(them);
    }

    println!("{}", score);
}
