use std::io;
use std::collections::{HashSet, VecDeque};

fn neighbors(position: (usize, usize), width: usize, height: usize) -> Vec::<(usize, usize)> {
    /* This could duplicate some positions, but that just means that
     * the duplicate would be skipped eventually because it would have
     * already been visited.
     */
    vec![
        (position.0, position.1.saturating_sub(1)),
        (position.0.saturating_sub(1), position.1),
        (position.0, position.1.saturating_add(1)),
        (position.0.saturating_add(1), position.1),
    ].into_iter()
    .filter(
        |p| p.0 < width && p.1 < height
    ).collect()
}

fn main() {
    let lines = io::stdin().lines();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, maybe_line) in lines.enumerate() {
        let mut row_vec = Vec::new();
        for (col, b) in maybe_line.unwrap().bytes().enumerate() {
            if b == 83 { // 'S'
                start = (col, row);
                row_vec.push(97); // 'a'
            } else if b == 69 { // 'E'
                end = (col, row);
                row_vec.push(122); // 'z'
            } else {
                row_vec.push(b);
            }
        }
        grid.push(row_vec);
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let width = grid[0].len();
    let height = grid.len();

    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (position, length) = queue.pop_front().unwrap();
        /* We already checked this one. Keep going. */
        if visited.contains(&position) {
            continue;
        }
        let current_height = grid[position.1][position.0];
        if position == end {
            println!("{}", length);
            break;
        }
        visited.insert(position);
        for neighbor in neighbors(position, width, height) {
            if !visited.contains(&neighbor) {
                let height = grid[neighbor.1][neighbor.0];
                /* If the neighbor is below us or is one above us,
                 * enqueue it to be visited.
                 */
                if height <= current_height || current_height + 1 == height {
                    queue.push_back((neighbor, length + 1));
                }
            }
        }
    }
}
