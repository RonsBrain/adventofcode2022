use std::io;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    AddValue(i32),
    AddOld,
    MulValue(i32),
    MulOld,
}

struct Monkey {
    item_queue: VecDeque<i32>,
    operation: Operation,
    divisor: i32,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

struct Sim {
    monkeys: Vec<Monkey>,
    inspected_counts: Vec<i32>,
}

impl Sim {
    fn parse() -> Self {
        let mut lines = io::stdin().lines().peekable();

        let mut monkeys = Vec::new();
        let mut inspected_counts = Vec::new();

        
        while lines.peek().is_some() {
            /* Consume the monkey number line */
            lines.next();

            /* Parse the worry numbers */
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split(": ");
            parts.next();
            let item_queue = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>();

            /* Parse operation */
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split(": ");
            parts.next();
            let mut op_parts = parts.next().unwrap().split(" ");
            /* Don't care about these parts */
            op_parts.next();
            op_parts.next();
            op_parts.next();
            let operation = match op_parts.next().unwrap() {
                "+" => {
                    match op_parts.next().unwrap().parse::<i32>() {
                        Ok(val) => Operation::AddValue(val),
                        _ => Operation::AddOld,
                    }
                }
                "*" => {
                    match op_parts.next().unwrap().parse::<i32>() {
                        Ok(val) => Operation::MulValue(val),
                        _ => Operation::MulOld,
                    }
                }
                _ => panic!("wut?")
            };

            /* Parse divisor */
            let line = lines.next().unwrap().unwrap();
            let divisor = line[21..].to_string().parse::<i32>().unwrap();

            /* Parse conditions */
            let monkey_if_true = lines.next().unwrap().unwrap()[29..].to_string().parse::<usize>().unwrap();
            let monkey_if_false = lines.next().unwrap().unwrap()[30..].to_string().parse::<usize>().unwrap();

            inspected_counts.push(0);
            let monkey = Monkey {
                item_queue,
                operation,
                divisor,
                monkey_if_true,
                monkey_if_false,
            };

            /* Add the monkey to the list */
            monkeys.push(monkey);
            /* Consume next line if there is one */
            lines.next();
        }

        Sim { monkeys, inspected_counts }
    }

    fn simulate(&mut self) -> i32 {
        for _ in 0..20 {
            /* I wanted to use an iterator or at least hold on to a reference
             * to the monkey, but that would continue holding that reference
             * for the duration of this loop iteration, meaning we can't pull
             * the destination monkey from the list as it also needs to be a
             * mutable reference. The thing to do is to reference the monkey
             * by index every time so that the mutable reference is released
             * after each use. Bleh.
             */
            for i in 0..self.monkeys.len() {
                for item in self.monkeys[i].item_queue.drain(0..).collect::<Vec<i32>>() {
                    self.inspected_counts[i] += 1;
                    let result = match self.monkeys[i].operation {
                        Operation::AddValue(v) => item + v,
                        Operation::AddOld => item + item,
                        Operation::MulValue(v) => item * v,
                        Operation::MulOld => item * item,
                    } / 3;
                    let dest = match result % self.monkeys[i].divisor == 0 {
                        true => self.monkeys[i].monkey_if_true,
                        false => self.monkeys[i].monkey_if_false,
                    };
                    self.monkeys[dest].item_queue.push_back(result);
                }
            }
        }

        self.inspected_counts.sort();
        self.inspected_counts.reverse();
        self.inspected_counts[0] * self.inspected_counts[1]
    }
}

fn main() {
    let mut sim = Sim::parse();
    println!("{}", sim.simulate());
}
