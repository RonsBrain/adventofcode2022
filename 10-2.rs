use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut scan_line = String::new();

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
            if cycle >= reg_x - 1 && cycle <= reg_x + 1 {
                scan_line.push('#');
            } else {
                scan_line.push(' ');
            }
            cycle += 1;
            if delay == 1 {
                reg_x += arg;
            }
            if cycle == 40 {
                println!("{}", scan_line);
                scan_line = String::new();
                cycle = 0;
            }
        }
        if cycle >= 220 {
            break;
        }
    }
}
