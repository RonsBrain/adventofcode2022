use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut current_size = Vec::new();
    let mut current = 0;
    let mut dir_sizes = Vec::new();

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
                                dir_sizes.push(current);
                                let last_size = current_size.pop().unwrap();
                                current = current + last_size;
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
    dir_sizes.push(current);
    current += current_size.into_iter().sum::<u32>();
    let remaining = 70000000 - current;
    let to_del = dir_sizes
        .iter()
        .filter(|x| *x + remaining >= 30000000)
        .min()
        .unwrap();
    println!("{}", to_del);
}
