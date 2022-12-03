use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut lines = io::stdin().lines();
    let mut total = 0;
    while let Some(chunk) = Some([lines.next(), lines.next(), lines.next()]) {
        if !chunk[0].is_some() {
            break;
        }
        let mut remaining = HashSet::new();
        for line in chunk {
            let unwrapped = line.unwrap();
            let temp = unwrapped.expect("Didn't get lines?");
            let bytes = temp.as_bytes();
            let sack: HashSet<u8> = HashSet::from_iter(bytes.iter().cloned());
            if remaining.len() == 0 {
                remaining.extend(&sack);
            } else {
                remaining = HashSet::from_iter(remaining.intersection(&sack).cloned())
            };
        }
        let unique = remaining.iter().next().unwrap();
        total = total + match *unique as char {
            'a'..='z' => (unique - 'a' as u8 + 1) as i32,
            'A'..='Z' => (unique - 'A' as u8 + 27) as i32,
            _ => panic!("You done goofed: {}", unique),
        };
    }

    println!("{}", total);
}
