use std::io;
use std::collections::VecDeque;

enum ParseState {
    BuildStacks,
    Skip,
    MoveStacks,
}

fn main() {
    use ParseState::*;
    let mut state = BuildStacks;
    let mut lines = io::stdin().lines().peekable();
    /* Peek at the first line to get how many stacks there are */
    let line: String = lines.peek().as_ref().unwrap().as_ref().unwrap().to_string();

    let num_stacks = (line.len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = (0..num_stacks)
        .map(|_| VecDeque::new())
        .collect();

    for line in lines {
        let unwrapped = line.unwrap();
        match state {
            BuildStacks => {
                /* We are done reading the stack arrangement. Skip the next line */
                if unwrapped.chars().nth(1).unwrap().is_ascii_digit() {
                    state = Skip;
                    continue;
                }
                let line_length = unwrapped.len();
                for i in (1..line_length).step_by(4) {
                    let x = unwrapped.chars().nth(i).unwrap();
                    if x != ' ' {
                        let stack = stacks.get_mut(i / 4).unwrap();
                        stack.push_front(x);
                    }
                }
            },
            Skip => state = MoveStacks,
            MoveStacks => {
                let mut segments = unwrapped.split(" ");
                let (number, from, to): (usize, usize, usize) = (
                    /* nth() consumes the iterator. These lines aren't getting the
                     * same index each time, but advancing the iterator two steps
                     * and getting that value.
                     */
                    segments.nth(1).unwrap().parse().unwrap(),
                    segments.nth(1).unwrap().parse().unwrap(),
                    segments.nth(1).unwrap().parse().unwrap(),
                );
                let from_stack = stacks.get_mut(from - 1).unwrap();
                let to_move: Vec<char> = from_stack.drain((from_stack.len() - number)..).collect();
                let to_stack = stacks.get_mut(to - 1).unwrap();
                for i in to_move.iter().rev() {
                    to_stack.push_back(*i);
                }
            },
        }
    }
    let output = stacks
        .iter()
        .map(|s| s.back().unwrap())
        .collect::<String>();
    println!("{}", output);
}
