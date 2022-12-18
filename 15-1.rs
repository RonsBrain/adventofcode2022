use std::io;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use std::ops::RangeInclusive;

fn parse_coords(input: &str) -> (i32, i32) {
    let mut parts = input.split(", ");
    let x = parts.next().unwrap()[2..].parse().unwrap();
    let y = parts.next().unwrap()[2..].parse().unwrap();
    (x, y)
}

fn distance(start: (i32, i32), end: (i32, i32)) -> i32 {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

fn range_union(ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    /* Get the ranges in order from left to right */
    let mut sorted = ranges.to_vec();
    sorted.sort_by(|l, r| l.start().cmp(r.start()));
    /* Build a new vec with all the merged ranges */
    let mut result = Vec::new();
    let mut maybe_current: Option<RangeInclusive<i32>> = None;

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
    let mut beacons = HashSet::new();

    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        let mut parts = line[10..].split(": closest beacon is at ");
        let sensor = parse_coords(parts.next().unwrap());
        let beacon = parse_coords(parts.next().unwrap());
        distances.insert(sensor, distance(sensor, beacon));
        beacons.insert(beacon);
    }
    /* Set up which row we want based off test data or real data */
    let row = match distances.len() == 14 {
        true => 10,
        false => 2000000
    };

    /* Find all the sensors that sensed this row */
    let in_range = distances
        .iter()
        .filter(|d| {
            let s = d.0;
            let l = d.1;
            distance(*s, (s.0, row)) <= *l
        })
        .collect::<Vec<(&(i32, i32), &i32)>>();

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
    
    /* Count how many columns the ranges occupy */
    let mut total: i32 = merged.iter().map(|r| *r.end() - *r.start()).sum();

    /* Remove any counts that may have counted a beacon */
    for col in beacons.iter().filter(|b| b.1 == row).map(|b| b.0) {
        for range in merged {
            if range.contains(&col) {
                total -= 1;
            }
        }
    }
    println!("{}", total);
}
