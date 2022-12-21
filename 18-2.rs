use std::io;
use std::collections::HashSet;
use std::cmp::{min, max};

fn main() {
    let mut coordinates = HashSet::new();
    /* Keep track of the lower and upper bounds of each axis.
     * No, starting with MAX..MIN is not a mistake. We want to
     * keep building the range based off min()..max(), the first
     * values we compare should automatically become the favored
     * range.
     *
     * Range must be inclusive!
     */
    let mut x_range = i32::MAX..=i32::MIN;
    let mut y_range = i32::MAX..=i32::MIN;
    let mut z_range = i32::MAX..=i32::MIN;

    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        let c: Vec<i32> = line.split(",").map(|n| n.parse().unwrap()).collect();
        coordinates.insert((c[0], c[1], c[2]));
        /* Recompute the ranges. */
        x_range = min(*x_range.start(), c[0] - 1)..=max(*x_range.end(), c[0] + 1);
        y_range = min(*y_range.start(), c[1] - 1)..=max(*y_range.end(), c[1] + 1);
        z_range = min(*z_range.start(), c[2] - 1)..=max(*z_range.end(), c[2] + 1);
    }

    let mut visited = HashSet::new();
    /* We're basically going to do a flood fill. Kick things off with the
     * coordinate of the beginning range of each axis.
     */
    let mut to_visit = vec![(*x_range.start(), *y_range.start(), *z_range.start())];

    let mut surface_area = 0;
    while !to_visit.is_empty() {
        /* Grab the next coordinate to visit. */
        let visiting = to_visit.pop().unwrap();
        if visited.contains(&visiting) {
            /* We already visited this one. Ignore. */
            continue;
        }
        /* Mark that we've visted this coordinate. */
        visited.insert(visiting);

        /* Look at all it's neighbors */
        for offset in [-1, 1] {
            for trial in [
                (visiting.0 + offset, visiting.1, visiting.2),
                (visiting.0, visiting.1 + offset, visiting.2),
                (visiting.0, visiting.1, visiting.2 + offset),
            ]
            {
                /* Are we inside the bounding box that contains the object? */
                if x_range.contains(&trial.0) &&
                   y_range.contains(&trial.1) &&
                   z_range.contains(&trial.2)
                {
                    if coordinates.contains(&trial) {
                        /* We've hit a piece of the object, which increases our
                         * surface area.
                         */
                        surface_area += 1;
                    } else {
                        /* We've hit empty space. Expand into this space. */
                        to_visit.push(trial);
                    }
                }
            }
        }
    }
    println!("{}", surface_area);
}
