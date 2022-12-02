use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut current = 0;
    let mut max = 0;
    for line in lines {
        match line.unwrap().parse::<i32>() {
            Ok(num) => {
                current += num;
            },
            _ => {
                if current > max {
                    max = current;
                }
                current = 0;
            }
        }
    }
    println!("{}", max);
}
