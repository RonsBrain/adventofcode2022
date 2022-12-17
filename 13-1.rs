use std::io;
use std::iter::zip;
use std::cmp::Ordering;

#[derive(Debug)]
enum List {
    Children(Vec<List>),
    Value(i32),
}

impl List {
    fn from_str(s: &str) -> List {
        if s.starts_with('[') {
            /* Parse a list */
            let s = &s[1..s.len() - 1]; // Assume an ending bracket
            let mut children = Vec::new();
            let mut depth = 0;
            let mut start = 0;

            if s.len() == 0 {
                return List::Children(children); // Empty list
            }

            for (i, c) in s.chars().enumerate() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 0 {
                            children.push(List::from_str(&s[start..i]));
                            start = i + 1;
                        }
                    }
                    _ => ()
                }
            }
            children.push(List::from_str(&s[start..]));
            List::Children(children)
        } else {
            /* Parse a single value */
            List::Value(s.parse().unwrap())
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        use List::*;
        match self {
            Children(v) => {
                match other {
                    Children(ov) => {
                        for (left, right) in zip(v, ov) {
                            if left < right {
                                /* left < right, in order */
                                return Ordering::Less;
                            } else if left > right {
                                /* left > right, out of order */
                                return Ordering::Greater;
                            }
                        }
                        if v.len() < ov.len() {
                            /* left < right, in order */
                            return Ordering::Less;
                        } else if v.len() > ov.len() {
                            /* left > right, out of order */
                            return Ordering::Greater;
                        } else {
                            /* Equal, still out of order */
                            return Ordering::Equal;
                        }
                    }
                    Value(ov) => {
                        /* Convert other to List and compare */
                        return self.cmp(&Children(vec![Value(*ov)]));
                    }
                }
            }
            Value(v) => {
                match other {
                    Children(_) => {
                        /* Convert self to list and compare */
                        return Children(vec![Value(*v)]).cmp(other);
                    }
                    Value(ov) => {
                        return v.cmp(ov);
                    }
                }
            }
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for List {
    /* YoU nEeD tO iMpLeMeNt Eq */
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn main() {
    let mut lines = io::stdin().lines().peekable();
    let mut index = 0;
    let mut index_sum = 0;

    while lines.peek().is_some() {
        index += 1;
        let mut pairs = Vec::new();
        for _ in 0..2 {
            let line = lines.next().unwrap().unwrap();
            let list = List::from_str(&line[..]);
            pairs.push(list);
        }
        if pairs[0] < pairs[1] {
            index_sum += index;
        }
        lines.next();
    }
    println!("{}", index_sum);
}
