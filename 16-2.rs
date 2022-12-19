use std::io;
use std::collections::HashMap;
use std::cmp::{min, max};

fn floyd_warshall(connections: &HashMap<String, (u64, u64, Vec<String>)>) -> HashMap<(&String, &String), u64> {
    /* https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm */

    /* Instead of building a V x V matrix, use a HashMap to build up the
     * distances as we traverse over the different vertices.
     */
    let mut dist = HashMap::new();
    
    /* Build the initial known distances */
    for (source, (_, _, conns)) in connections {
        for connection in conns {
            dist.insert((source, connection), 1);
            dist.insert((connection, source), 1);
            dist.insert((source, source), 0);
        }
    }

    for k in connections.keys() {
        for i in connections.keys() {
            for j in connections.keys() {
                /* Get the existing distances if known, otherwise resort
                 * to some maximum value.
                 */
                let current = match dist.get(&(i, j)) {
                    Some(v) => v,
                    _ => &u64::MAX
                };
                let a = match dist.get(&(i, k)) {
                    Some(v) => v,
                    _ => &u64::MAX
                };
                let b = match dist.get(&(k, j)) {
                    Some(v) => v,
                    _ => &u64::MAX
                };
                /* Protect against an overflow here */
                let (total_dist, overflow) = a.overflowing_add(*b);
                if !overflow {
                    dist.insert((i, j), min(*current, total_dist));
                } else {
                    dist.insert((i, j), *current);
                }
            }
        }
    }
    dist
}

fn visit(
    cave: &String,
    remaining: u64,
    current_flow: u64,
    caves_with_valves: &Vec<(&String, u64, u64)>,
    distances: &HashMap<(&String, &String), u64>,
    open_state: u64,
    result: &mut HashMap<u64, u64>
) {
    /* Get the current best flow for the given open valves */
    let total_flow = match result.get(&open_state) {
        Some(f) => *f,
        _ => 0,
    };

    result.insert(open_state, max(total_flow, current_flow));

    /* Visit all the caves */
    for (next_cave, cave_flow, cave_mask) in caves_with_valves
        .iter()
        /* Filter any caves that have an open valve, or the current cave */
        .filter(|(k, _, m)| (m & open_state) == 0 && *k != cave)
    {
        let dist = distances.get(&(&cave, &next_cave)).unwrap();
        let (new_remaining, overflow) = remaining.overflowing_sub(dist + 1);
        /* If moving to new_cave doesn't burn our time, visit it */
        if !overflow && new_remaining > 0 {
            /* NOTE: Update the current flow with this valve's flow
             * multiplied by the remaining time, since the valve will flow
             * for the remainder of the time.
             */
            visit(
                next_cave,
                new_remaining,
                current_flow + new_remaining * cave_flow, 
                caves_with_valves,
                distances,
                open_state | cave_mask,
                result
            );
        }
    }
}

fn main() {
    let mut connections_map = HashMap::new();
    let mut next_mask: u64 = 1;
    for maybe_line in io::stdin().lines() {
        let line = maybe_line.unwrap();
        // Break up cave identifier and cave connections
        let mut parts = line.split("; ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        // Cave identifier is two characters only */
        let valve = left[6..8].to_string();
        // Flow is at the end of cave identifier
        let flow: u64 = left[23..].parse().unwrap();
        // If connection lilst is even, there is only one cave.
        let connection_list = match right.len() % 2 {
            0 => right[22..].split(", "),
            _ => right[23..].split(", "),
        };
        let current_connections = connection_list
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        // Only set mask on caves with valves
        let mask = match flow > 0 {
            true => { next_mask = next_mask << 1; next_mask >> 1 },
            false => 0,
        };
        connections_map.insert(valve.clone(), (flow, mask, current_connections));
    }

    let dist = floyd_warshall(&connections_map);

    /* Build a vec of caves, filtering out any caves that have no valve */
    let connections = connections_map
        .iter()
        .filter(|(_, (f, _, _))| *f > 0)
        .map(|(k, (f, m, _))| (k, *f, *m))
        .collect::<Vec<(&String, u64, u64)>>();
    let mut result = HashMap::new();
    visit(&String::from("AA"), 26, 0, &connections, &dist, 0, &mut result);
    let mut total = 0;
    /* Find all the combinations with two different sents of opened valves.
     * This means the elephant opened one set of valves and you opened the other.
     * If the total flow from both of these is greater than the current max,
     * then update the max.
     */
    for (opened, flow) in result.iter() {
        for (opened2, flow2) in result.iter() {
            if (opened & opened2) == 0 {
                /* Distinct valves opened */
                if flow + flow2 > total {
                    total = flow + flow2;
                }
            }
        }
    }
    println!("{}", total);
}
