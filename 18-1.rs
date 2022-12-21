use std::io;
use std::collections::HashSet;

fn main() {
    let mut coordinates = HashSet::new();
    /* BUild a set out of all the cooridnates */
    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        let c: Vec<i32> = line.split(",").map(|n| n.parse().unwrap()).collect();
        coordinates.insert((c[0], c[1], c[2]));
    }

    let mut surface_area = 0;
    for coordinate in coordinates.iter() {
        let mut neighbor_count = 0;
        /* For each cooridnate, look at all its neightbors */
        for offset in [-1, 1] {
            for trial in [
                (coordinate.0 + offset, coordinate.1, coordinate.2),
                (coordinate.0, coordinate.1 + offset, coordinate.2),
                (coordinate.0, coordinate.1, coordinate.2 + offset),
            ]
            {
                /* If ther is a neightbor, count it */
                if coordinates.contains(&trial) {
                    neighbor_count += 1;
                }
            }
        }
        /* Surface area increases by the number of neightbors
         * the coordinate DOESN'T have.
         */
        surface_area += 6 - neighbor_count;
    }
    println!("{}", surface_area);
}
