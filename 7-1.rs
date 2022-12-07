use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut current_size = Vec::new();
    let mut current = 0;
    let mut total = 0;

    for maybe_line in lines {
        let line = maybe_line.unwrap();
        match &line[0..1] {
            "$" => {
                match &line[2..4] {
                    "cd" => {
                        let dirname = &line[5..];
                        match dirname {
                            "/" => {
                                current_size.clear();
                            }
                            ".." => {
                                if current < 100000 {
                                    total += current;
                                }
                                current = current + current_size.pop().unwrap();
                            }
                            _ => {
                                current_size.push(current);
                                current = 0;
                            }
                        }
                    }
                    /* Don't care about ls command */
                    _ => (),
                }
            },
            _ => {
                match &line[0..3] {
                    /* Don't care about directories */
                    "dir" => (),
                    _ => {
                        current += line.split(" ").next().unwrap().parse::<u32>().unwrap();
                    }
                }
            }
        }
    }
    println!("{}", total);
}
