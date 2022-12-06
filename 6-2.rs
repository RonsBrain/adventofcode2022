use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => panic!("Can't read input"),
        _ => ()
    }

    let mut offset = 14;
    for chunk in input.chars().collect::<Vec<char>>().windows(14) {
        let mut s = HashSet::new();
        for i in chunk {
            s.insert(i);
        }
        println!("{:?}", s);
        if s.len() == 14 {
            break;
        } else {
            offset += 1;
        }
    }

    println!("{}", offset);
}
