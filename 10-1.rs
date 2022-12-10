use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut cycle = 1;
    let mut reg_x = 1;
    let mut sum_strengths = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let arg = match parts.next() {
            Some(x) => x.parse::<i32>().unwrap(),
            _ => 0,
        };

        let mut cycles = 1;

        match command {
            "addx" => {
                cycles = 2;
            },
            _ => {
            },
        }

        for delay in 0..cycles {
            cycle += 1;
            if delay == 1 {
                reg_x += arg;
            }
            if (cycle - 20) % 40 == 0 {
                sum_strengths += cycle * reg_x;
            }
        }
        if cycle >= 220 {
            break;
        }
    }
    println!("{}", sum_strengths);
}
