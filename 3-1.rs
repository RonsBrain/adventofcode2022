use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        /* Need to store this temporarily in order to reference it later */
        let bytes = unwrapped.as_bytes();
        /* Clone the iterators so the HashSet has values and not references */
        let left: HashSet<u8> = HashSet::from_iter(bytes[..bytes.len() / 2].iter().cloned());
        let right: HashSet<u8> = HashSet::from_iter(bytes[bytes.len() / 2..].iter().cloned());
        let unique = left.intersection(&right).next().unwrap();
        total = total + match *unique as char {
            'a'..='z' => (unique - 'a' as u8 + 1) as i32, // Start priority at 1
            'A'..='Z' => (unique - 'A' as u8 + 27) as i32, // Start priortiy as 27
            _ => panic!("You done goofed: {}", unique),
        };
    }

    println!("{}", total);
}
