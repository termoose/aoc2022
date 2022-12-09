use std::fs;
use std::collections::HashSet;

fn packet_marker(length: usize, msg: &str) -> i32 {
    let result: Vec<(usize, bool)> = msg
        .as_bytes()
        .windows(length)
        .enumerate()
        .map(|(i, w)| {
            let m: HashSet<&u8> = w.into_iter().collect();
            (i + length, m.len() == length)
        })
        .filter(|(_, b)| *b)
        .take(1)
        .collect();

    result[0].0 as i32
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    println!("A: {}", packet_marker(4, &content));
    println!("B: {}", packet_marker(14, &content));
}
