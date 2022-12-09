use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut grid = Vec::new();
    let mut max_score = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        let row = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect::<Vec<u8>>();

        grid.push(row);
    }

    for (row, heights) in grid.iter().enumerate() {
        if row == 0 || row == grid.len() - 1 {
            continue;
        }

        for (col, height) in heights.iter().enumerate() {
            if col == 0 || col == heights.len() - 1 {
                continue;
            }

            if *height == 0 {
                continue;
            }

            /* I tried to find a functional way to do all of the following
             * but got tripped up when the iterator needed to stop when the
             * current height was the same as the iterated height.
             */
            let mut left = 0;
            /* Iterate in reverse order, right to left */
            for h in heights[0..col].iter().rev() {
                left += 1;
                if height <= h {
                    break;
                }
            }
            let mut right = 0;
            for h in heights[col + 1..].iter() {
                right += 1;
                if height <= h {
                    break;
                }
            }
            let mut up = 0;
            /* Iterate in reverse order, going up.
             * For each row, get the col'th item
             */
            for h in grid[0..row].iter().map(|h| h[col]).rev() {
                up += 1;
                if height <= &h {
                    break;
                }
            }
            let mut down = 0;
            /* For each row, get the col'th item */
            for h in grid[row + 1..].iter().map(|h| h[col]) {
                down += 1;
                if height <= &h {
                    break;
                }
            }
            let score = left * right * up * down;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("{}", max_score);
}
