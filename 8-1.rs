use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut grid = Vec::new();
    let mut total_visible = 0;

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
            total_visible += grid.len();
            continue;
        }

        for (col, height) in heights.iter().enumerate() {
            if col == 0 || col == heights.len() - 1 {
                total_visible += 1;
                continue;
            }

            if *height == 0 {
                continue;
            }

            if height > heights[0..col] /* Anything to the left */
                .iter()
                .max()
                .unwrap() ||
            height > heights[col + 1..] /* Anything to the right */
                .iter()
                .max()
                .unwrap() ||
            height > &grid[0..row] /* Anything above */
                .iter()
                .map(|h| h[col]) /* Get col'th item in row */
                .max()
                .unwrap() ||
            height > &grid[row + 1..] /* Anything below */
                .iter()
                .map(|h| h[col]) /* Get col'th item in row */
                .max()
                .unwrap()
            {
                total_visible += 1;
            }
        }
    }
    println!("{}", total_visible);
}
