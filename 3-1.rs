use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let lines = io::stdin().lines();
    let mut total = 0;
    for line in lines {
        let unwrapped = line.unwrap();
        let bytes = unwrapped.as_bytes();
        let left: HashSet<u8> = HashSet::from_iter(bytes[..bytes.len() / 2].iter().cloned());
        let right: HashSet<u8> = HashSet::from_iter(bytes[bytes.len() / 2..].iter().cloned());
        let unique = left.intersection(&right).next().unwrap();
        total = total + match *unique as char {
            'a'..='z' => (unique - 'a' as u8 + 1) as i32,
            'A'..='Z' => (unique - 'A' as u8 + 27) as i32,
            _ => panic!("You done goofed: {}", unique),
        };
    }

    println!("{}", total);
}
