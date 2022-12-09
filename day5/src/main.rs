use std::fs;
use std::collections::{BTreeMap};

struct Cargo {
    state: BTreeMap<i32, Vec<char>>,
}

// (count, from, to)
type Move = (i32, i32, i32);

impl Cargo {
    fn new(init: &str) -> Cargo {
        let mut state: BTreeMap<i32, Vec<char>> = BTreeMap::new();
        let mut lines: Vec<&str> = init.split("\n").collect();

        lines.pop();
        lines.iter().for_each(|l| {
            for (i, c) in l.chars().enumerate() {
                if !c.is_alphabetic() {
                    continue;
                }

                let index: i32 = ((i - 1) / 4 + 1) as i32;
                let stack = &mut state.get(&index);

                if stack.is_none() {
                    state.insert(index, vec![c]);
                } else {
                    let mut newstack = stack.unwrap().clone();
                    newstack.push(c);
                    state.insert(index, newstack);
                }
            }

        });

        Cargo { state }
    }

    fn move_crate_9000(&mut self, from: i32, to: i32) {
        let from_stack = self.state.get(&from);

        if from_stack.is_some() {
            let mut fs = from_stack.unwrap().clone();
            let elem = fs.first().unwrap().clone();
            fs.remove(0);

            self.state.insert(from, fs);

            let to_stack = self.state.get(&to);

            if to_stack.is_some() {
                let mut ts = to_stack.unwrap().clone();
                ts.insert(0, elem);

                self.state.insert(to, ts);
            }
        }
    }

    fn move_crate_9001(&mut self, count: i32, from: i32, to: i32) {
        let from_stack = self.state.get(&from);

        if from_stack.is_some() {
            let fs = from_stack.unwrap().clone();
            let mut crates: Vec<char> = fs[0..(count as usize)].to_vec();
            let remaining: Vec<char> = fs[(count as usize)..].to_vec();

            self.state.insert(from, remaining);
            //println!("fs: {:?}", fs);
            //println!("{:?}", crates);

            let to_stack = self.state.get(&to);
            if to_stack.is_some() {
                let ts = to_stack.unwrap().clone();
                crates.extend(&ts);
                //println!("{:?}", crates);
                self.state.insert(to, crates);

                //println!("{:?}", self.state);
            }
        }
    }

    fn move_crates_9000(&mut self, count: i32, from: i32, to: i32) {
        for _ in 0..count {
            self.move_crate_9000(from, to);
        }
    }

    fn line_to_move(line: &str) -> Move {
        let words: Vec<&str> = line.split(" ").collect();
        let count = words[1].parse::<i32>().unwrap();
        let from = words[3].parse::<i32>().unwrap();
        let to = words[5].parse::<i32>().unwrap();
        (count, from, to)
    }

    fn print_solution(&self) {
        for (_, v) in &self.state {
            print!("{}", v.first().unwrap());
        }
        println!();
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = content.trim_end().split("\n\n").collect();

    let mut cargo_9000 = Cargo::new(parts[0]);
    parts[1].split("\n").for_each(|l| {
        let command = Cargo::line_to_move(l);
        cargo_9000.move_crates_9000(command.0, command.1, command.2);
    });
    print!("A: ");
    cargo_9000.print_solution();

    let mut cargo_9001 = Cargo::new(parts[0]);
    parts[1].split("\n").for_each(|l| {
        let command = Cargo::line_to_move(l);
        cargo_9001.move_crate_9001(command.0, command.1, command.2);
    });
    print!("B: ");
    cargo_9001.print_solution();
}
