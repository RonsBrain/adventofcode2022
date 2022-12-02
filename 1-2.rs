use std::io;

fn update_max(current: Vec<i32>, new: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut item = new;
    for i in current.iter() {
        if item > *i {
            result.push(item);
            item = *i;
        } else {
            result.push(*i);
        }
    }
    result
}

fn main() {
    let lines = io::stdin().lines();
    let mut current = 0;
    let mut max = vec![0, 0, 0];
    for line in lines {
        match line.unwrap().parse::<i32>() {
            Ok(num) => {
                current += num;
            },
            _ => {
                max = update_max(max, current);
                current = 0;
            }
        }
    }
    max = update_max(max, current);
    println!("{}", max.iter().sum::<i32>());
}
