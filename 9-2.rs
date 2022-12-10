use std::io;
use std::collections::HashSet;

fn main() {
    let lines = io::stdin().lines();

    let mut knots = [(0i32, 4i32);10];
    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let mut parts = line.split(" ");
        let direction = parts.next().unwrap();
        let length = parts.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..length {
            let head = &knots[0];
            match direction {
                "R" => knots[0] = (head.0 + 1, head.1),
                "U" => knots[0] = (head.0, head.1 - 1),
                "L" => knots[0] = (head.0 - 1, head.1),
                "D" => knots[0] = (head.0, head.1 + 1),
                _ => panic!("wut?")
            }
            for i in 1..10 {
                let head = knots[i - 1];
                let tail = knots[i];
                let to_travel = (head.0 - tail.0, head.1 - tail.1);
                let distance = to_travel.0.abs() + to_travel.1.abs(); // Suck it Pythagoras

                if distance == 2 {
                    /* See if the tail is more than 2 units away,
                     * or if we're at some sort of diagonal.
                     */
                    if to_travel.0.abs() == 2 {
                        /* Need to move tail closer horizontally */
                        knots[i] = (tail.0 + to_travel.0 / 2, tail.1);
                    } else if to_travel.1.abs() == 2 {
                        /* Need to move tail closer vertically */
                        knots[i] = (tail.0, tail.1 + to_travel.1 / 2);
                    }
                    /* else { We're at a touching diagonal. Do nothing. } */
                } else if distance > 2 {
                    /* Head is at a diagonal but not touching, so figure out
                     * the skewed axis and alter the tail based on that.
                     */
                    let delta_x = match to_travel.0.abs() == 2 {
                        true => to_travel.0 / 2, // Skewed horizontally
                        false => to_travel.0,
                    };

                    let delta_y = match to_travel.1.abs() == 2 {
                        true => to_travel.1 / 2, // Skewed vertically
                        false => to_travel.1,
                    };

                    knots[i] = (tail.0 + delta_x, tail.1 + delta_y);
                }
            }
            visited.insert(knots[9]);
        }
    }
    println!("{}", visited.len());
}
