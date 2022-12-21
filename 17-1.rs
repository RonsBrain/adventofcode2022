use std::io;
use std::collections::HashSet;

/* Make some rocks. Represent the rocks as the offsets related
 * to which parts make up the rock, offset from the lower left
 * side of the rock.
 */
fn build_rocks() -> Vec<Vec<(i32, i32)>> {
    let mut rocks = Vec::new();
    rocks.push(vec![(0, 0), (1, 0), (2, 0), (3, 0)]); // Horizontal bar
    rocks.push(vec![(1,0), (0, 1), (1, 1), (2, 1), (1, 2)]); // Cross
    rocks.push(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]); // L-shape
    rocks.push(vec![(0, 0), (0, 1), (0, 2), (0, 3)]); // Vertical bar
    rocks.push(vec![(0, 0), (1, 0), (0, 1), (1, 1)]); // Swuare
    rocks
}

fn test_new_position(
    rock: &Vec<(i32, i32)>,
    position: &(i32, i32),
    channel: &HashSet<(i32, i32)>,
    offsets: (i32, i32)
) -> bool {
    /* Iterate all the pieces of the rock, adjust for the
     * current rock position, and see if those coordinates
     * are in the channel. If so, there is a collision.
     */
    for piece in rock.iter() {
        let next = (
            position.0 + offsets.0 + piece.0,
            position.1 + offsets.1 + piece.1
        );
        if channel.contains(&next) {
            return false;
        }
        if next.0 < 0 || next.0 > 6 || next.1 < 0 {
            return false;
        }
    }
    true
}

fn put_rock(
    rock: &Vec<(i32, i32)>,
    position: &(i32, i32),
    channel: &mut HashSet<(i32, i32)>,
) {
    /* Adjust each rock piece for the position and insert that
     * position into the channel.
     */
    for piece in rock.iter() {
        channel.insert((position.0 + piece.0, position.1 + piece.1));
    }
}

fn main() {
    let rocks = build_rocks();
    let mut rocks_iter = rocks.iter().cycle(); 
    let mut channel = HashSet::new();
    let jets = io::stdin().lines().next().unwrap().unwrap();
    let mut jets_iter = jets.chars().cycle();
    let mut maybe_rock = None;
    let mut rock_count = 0;
    let mut position = (0, 0);
    let mut top = 0;

    while rock_count <= 2022 {
        if !maybe_rock.is_some() {
            /* We don't have an active rock. Get one. */
            maybe_rock = Some(rocks_iter.next().unwrap());
            position = (2, top + 3);
            rock_count += 1;
        }

        let current_rock = maybe_rock.unwrap();
        let move_offset = match jets_iter.next().unwrap() {
            '<' => -1,
            '>' => 1,
            _ => panic!("???")
        };

        /* If we can move in the jet direction, do it. */
        if test_new_position(&current_rock, &position, &channel, (move_offset, 0)) {
            position = (position.0 + move_offset, position.1);
        }

        /* If we can move down, do it. */
        if test_new_position(&current_rock, &position, &channel, (0, - 1)) {
            position = (position.0, position.1 - 1);
        } else {
            /* Otherwise, the rock is fully dropped and we should insert it
             * into the channel.
             */
            put_rock(&current_rock, &position, &mut channel);
            maybe_rock = None;

            /* The top is the highest rock piece in the channel. */
            top = channel.iter()
                .map(|(_, r)| r)
                .max()
                .unwrap() + 1;
        }
    }
    println!("{}", top);
}
