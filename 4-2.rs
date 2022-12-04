use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        let assignments: Vec<&str> = unwrapped.split(",").collect();
        let first: Vec<&str> = assignments[0].split("-").collect();
        let second: Vec<&str> = assignments[1].split("-").collect();
        /* Use the built in std::ops::Range type to make things easier */
        let first_range = first[0].parse::<i32>().unwrap()..=first[1].parse::<i32>().unwrap();
        let second_range = second[0].parse::<i32>().unwrap()..=second[1].parse::<i32>().unwrap();
        if (
            first_range.contains(&second_range.start())
            || first_range.contains(&second_range.end())
        ) || (
            second_range.contains(&first_range.start())
            || second_range.contains(&first_range.end())
        ) {
            total += 1;
        }
    }
    println!("{}", total);
}
