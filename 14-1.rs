use std::io;
use std::collections::HashSet;

fn main() {
    let mut grid = HashSet::new();
    let mut depth = 0;
    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        let points = line
            .split(" -> ")
            .map(|p| {
                let mut parts = p.split(",");
                let col = parts.next().unwrap().parse().unwrap();
                let row = parts.next().unwrap().parse().unwrap();
                (col, row)
            })
            .collect::<Vec<(i32, i32)>>();
        for pair in points.windows(2) {
            let mut sorted = pair.to_vec();
            sorted.sort();
            match sorted[0].0 != sorted[1].0 {
                true => {
                    for col in sorted[0].0..=sorted[1].0 {
                        grid.insert((col, sorted[0].1));
                    }
                    if sorted[0].1 > depth {
                        depth = sorted[0].1;
                    }
                }
                false => {
                    for row in sorted[0].1..=sorted[1].1 {
                        grid.insert((sorted[0].0, row));
                    }
                    if sorted[1].1 > depth {
                        depth = sorted[1].1;
                    }
                }
            };
        }
    }
    let mut total_sand = 0;
    loop {
        let mut sand = (500, 0);
        total_sand += 1;
        while sand.1 <= depth {
            let next_row = sand.1 + 1;
            if grid.contains(&(sand.0, next_row)) {
                if grid.contains(&(sand.0 - 1, next_row)) {
                    if grid.contains(&(sand.0 + 1, next_row)) {
                        break;
                    } else {
                        sand = (sand.0 + 1, next_row);
                    }
                } else {
                    sand = (sand.0 - 1, next_row);
                }
            } else {
                sand = (sand.0, next_row);
            }
        }
        if sand.1 > depth {
            break;
        } else {
            grid.insert(sand);
        }
    }
    println!("{}", total_sand - 1);
}
