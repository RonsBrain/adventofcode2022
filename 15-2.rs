use std::io;
use std::collections::HashMap;
use std::cmp::{min, max};
use std::ops::RangeInclusive;

fn parse_coords(input: &str) -> (i64, i64) {
    let mut parts = input.split(", ");
    let x = parts.next().unwrap()[2..].parse().unwrap();
    let y = parts.next().unwrap()[2..].parse().unwrap();
    (x, y)
}

fn distance(start: (i64, i64), end: (i64, i64)) -> i64 {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

fn range_union(ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    /* Get the ranges in order from left to right */
    let mut sorted = ranges.to_vec();
    sorted.sort_by(|l, r| l.start().cmp(r.start()));
    /* Build a new vec with all the merged ranges */
    let mut result = Vec::new();
    let mut maybe_current: Option<RangeInclusive<i64>> = None;

    for range in sorted {
        match maybe_current {
            None => maybe_current = Some(range), // Set the current to this range
            Some(current) => {
                /* Should we expand the current? We should if the current
                 * overlaps the range or is adjacent to the range.
                 */
                if current.contains(range.start()) || current.contains(&(range.start() - 1)) || current.contains(range.end()) {
                    maybe_current = Some(
                        min(*range.start(), *current.start())..=max(*range.end(), *current.end())
                    );
                } else {
                    /* Nope. Out of range. Push it into the result
                     * and get ready for the next range.
                     */
                    result.push(current);
                    maybe_current = None;
                }
            }
        }
    }
    if maybe_current.is_some() {
        result.push(maybe_current.unwrap());
    }
    result
}

fn main() {
    let mut distances = HashMap::new();
    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;

    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        let mut parts = line[10..].split(": closest beacon is at ");
        let sensor = parse_coords(parts.next().unwrap());
        let beacon = parse_coords(parts.next().unwrap());
        distances.insert(sensor, distance(sensor, beacon));
        min_row = min(min_row, sensor.1);
        max_row = max(max_row, sensor.1);
    }

    for row in min_row..=max_row {
        /* Find all the sensors that sensed this row */
        let in_range = distances
            .iter()
            .filter(|d| {
                let s = d.0;
                let l = d.1;
                distance(*s, (s.0, row)) <= *l
            })
            .collect::<Vec<(&(i64, i64), &i64)>>();

        /* Compute the ranges for all the relevant sensors in this row */
        let disallowed = in_range
            .iter()
            .map(|r| {
                let (sensor, d) = r;
                let dist = *d - (row - sensor.1).abs();
                let left = sensor.0 - dist;
                let right = sensor.0 + dist;
                left..=right
            })
            .collect();

        /* Merge all the ranges together */
        let merged = &range_union(disallowed);
        
        if merged.len() > 1 {
            let left = &merged[0];
            println!("{}", (left.end() + 1) * 4000000 + row);
            break;
        }
    }
}
