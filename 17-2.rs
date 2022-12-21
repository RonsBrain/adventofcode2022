use std::io;
use std::collections::HashMap;
use std::iter::zip;
use std::cmp::max;

enum JetDirection {
    Left,
    Right,
}

fn main() {
    /* Represent the rocks as byte sequences
     * Go from the bottom up so it's easier to iterate and
     * add to the chamber.
     */
    let rocks: Vec<Vec<u8>> = vec![
        vec![0b00111100], // Horizontal bar
        vec![0b00010000, 0b00111000, 0b00010000], // Cross
        vec![0b00111000, 0b00001000, 0b00001000], // L-shape
        vec![0b00100000, 0b00100000, 0b00100000, 0b00100000], // Vertical bar
        vec![0b00110000, 0b00110000] // Square
    ];

    let mut chamber = vec![1u8;4];

    /* The rock formations repeat, and we want to keep track
     * of these repitions, so we will use a cache to watch
     * for these cycles.
     */
    let mut cycle_cache = HashMap::new();

    let line = io::stdin().lines().next().unwrap().unwrap();
    let jet_len = line.len() as u64;

    /* Build a nice mapping of our input to a direction enum */
    let mut jet_sequence = line
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => panic!("???")
        })
        .cycle(); // Repeat this forever!

    let rock_sequence = rocks.iter().cycle(); // Cycle through the rocks forever
    let mut top = 0;

    let mut rocks_added: u64 = 0;

    /* Keep track of which index we are in the infinite iterator. */
    let mut rock_num: u64 = 0;
    let mut jet_num: u64 = 0;

    let target = 1000000000000;
    let mut accumulated_height = 0;

    for next_rock in rock_sequence {
        /* These values are distinct. rock_num keeps track of where we
         * are in the rock iterator. rocks_added may increase by more
         * than 1 if a cycle is detected, which would skew the rock
         * number.
         */
        rock_num += 1;
        rocks_added += 1;
        if rocks_added > target {
            /* We're done! */
            break;
        }

        /* Make sure thers is enough room in the chamber to
         * start the rock falling.
         */
        while chamber.len() < top + 3 + next_rock.len() {
            /* Push 0b00000001 so that the least bit can act as
             * a guard when rotating.
             */
            chamber.push(1);
        }

        /* Keep track of where the rock currently is */
        let mut position = top + 3;

        let mut current_rock = next_rock.clone();
        /* Do all the movements until the rock has fully dropped. */
        loop {
            let direction = jet_sequence.next().unwrap();
            jet_num += 1;
            let end_position = position + current_rock.len();

            /* Do the jet movement */
            let next_move = zip(current_rock.iter(), chamber[position..end_position].iter())
                .map(|(p, c)| {

                    match direction {
                        JetDirection::Left => (p.rotate_left(1), *c),
                        JetDirection::Right => (p.rotate_right(1), *c)
                    }
                })
                .filter(|(p, c)| {
                    /* Filter out any rock part that is touching the chamber.
                     *
                     * If the piece is on the far left and is rotated left,
                     * the high order bit would fall into the low order bit
                     * and the guard bit in the channel would collide.
                     *
                     * If the piece is on the far right of the channel and is
                     * rotated right, the second least bit would then be the
                     * least bit and the guard bit in the channel would collide.
                     */
                    (p & *c) == 0
                })
                .map(|(p, _)| p)
                .collect::<Vec<u8>>();
            /* If the above has filtered out a rock part, that means that
             * that part collided with something in the chamber, so it
             * should not be affected by the jet.
             */
            if next_move.len() == current_rock.len() {
                current_rock = next_move;
            }

            /* Are we on the floor? Consider the rock dropped. */
            if position == 0 {
                break;
            }


            /* Do the drop. Test the rock against the chamber one position down. */
            let next_move = zip(
                    current_rock.iter(),
                    chamber[position - 1..end_position - 1].iter()
                )
                .filter(|(p, c)| {
                    /* Filter out any piece that would collide
                     * with the chamber.
                     */
                    (*p & *c) == 0
                })
                .map(|(p, _)| *p)
                .collect::<Vec<u8>>();

            /* If no piece was filtered out, then the rock can be moved down.
             *
             * Otherwise, the rock has collided with something below it,
             * so it should be considered fully dropped. End the movement.
             */
            if next_move.len() == current_rock.len() {
                position -= 1;
            } else {
                break;
            }
        };

        /* Place the rock into the chamber. */
        for (i, p) in (position..position + current_rock.len()).enumerate() {
            let t = chamber.get_mut(p).unwrap();
            *t = *t | current_rock[i];
        }

        /* Keep track of where the top is. */
        top = max(top, position + current_rock.len());

        /* Clear out any unused chamber parts at the top of the chamber. */
        while chamber.len() > top {
            chamber.pop();
        }

        /* Build a chamber state from the top 8 parts of the chamber. This will
         * be used to try to find cycles.
         *
         * This basically takes the last 8 bytes from the chamber and turns
         * them into a 64-bit number.
         */
        let chamber_state = match chamber.len() == 0 {
            true => None,
            false => Some(
                chamber
                    .iter()
                    .rev()
                    .take(8)
                    .fold(0u64, |state, curr| (state << 8) | *curr as u64)
                )
        };

        /* Compute a cache key from the chamber state, the rock number, and
         * the jet number.
         */
        let cache_key = (chamber_state, rock_num % 5, jet_num % jet_len);

        /* If we have seen these three values at the same time before, then
         * we have detected a cycle and can use this to our advantage.
         */
        if let Some((cycle_rocks_added, cycle_height)) = cycle_cache.get(&cache_key) {
            /* How many rocks did we add since this cycle was detected? */
            let rocks_to_add = rocks_added - cycle_rocks_added;

            /* How many cycles can we do to get to our target? */
            let total_cycles = (target - rocks_added) / rocks_to_add;

            /* Add all the rocks we can based off the cycle info. */
            rocks_added += rocks_to_add * total_cycles;

            /* Add all the height we can, knowing how many cycles. */
            accumulated_height += total_cycles * (top as u64 - cycle_height);

            /* Clear the cache so we can try to detect a different cycle. */
            cycle_cache.clear();
        }

        /* Keep track of the number of rocks added and where the top is for
         * this iteration.
         */
        cycle_cache.insert(cache_key, (rocks_added, top as u64));
    }

    /* We're done! The total height is the accumulated height plus whatever we
     * added after the last cycle.
     */
    println!("{}", top as u64 + accumulated_height);
}
