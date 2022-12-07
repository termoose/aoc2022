use std::fs;
use std::collections::BinaryHeap;

fn solve(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();

    let mut elf_calories: BinaryHeap<i32> = content
        .trim_end()
        .split("\n\n")
        .map(|e|
             e.split('\n')
             .map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .collect();

    println!("A: {:?}", elf_calories.peek().unwrap());

    let b = (0..3).map(|_| elf_calories.pop().unwrap()).sum::<i32>();
    println!("B: {:?}", b);
}

fn main() {
    solve("input.txt")
}
