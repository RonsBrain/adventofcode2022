use std::io;
use std::collections::HashSet;

fn main() {
    let lines = io::stdin().lines();

    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let length = parts.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..length {
            match direction {
                "R" => head = (head.0 + 1, head.1),
                "U" => head = (head.0, head.1 + 1),
                "L" => head = (head.0 - 1, head.1),
                "D" => head = (head.0, head.1 - 1),
                _ => panic!("wut?")
            }
            let to_travel = (head.0 - tail.0, head.1 - tail.1);
            let distance = to_travel.0.abs() + to_travel.1.abs(); // Suck it Pythagoras

            if distance == 2 {
                /* See if the tail is more than 2 units away,
                 * or if we're at some sort of diagonal.
                 */
                if to_travel.0.abs() == 2 {
                    /* Need to move tail closer horizontally */
                    tail = (tail.0 + to_travel.0 / 2, tail.1);
                } else if to_travel.1.abs() == 2 {
                    /* Need to move tail closer vertically */
                    tail = (tail.0, tail.1 + to_travel.1 / 2);
                }
                /* else { We're at a touching diagonal. Do nothing. } */
            } else if to_travel.0.abs() + to_travel.1.abs() > 2 {
                /* Head is at a diagonal but not touching, so figure out
                 * the skewed axis and alter the tail based on that.
                 */
                if to_travel.0.abs() == 2 {
                    /* Head is farther horizontally than vertically */
                    tail = (tail.0 + to_travel.0 / 2, tail.1 + to_travel.1);
                } else if to_travel.1.abs() == 2 {
                    /* Head is farther vertically than horizontally */
                    tail = (tail.0 + to_travel.0, tail.1 + to_travel.1 / 2);
                }
            }
            visited.insert(tail);
        }
    }
    println!("{}", visited.len());
}
