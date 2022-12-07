use std::fs;

fn opponent_shape_to_int(shape: char) -> i32 {
    shape as i32 - 'A' as i32
}

fn my_shape_to_int(shape: char) -> i32 {
    shape as i32 - 'X' as i32
}

fn score(line: &str) -> i32 {
    let opponent = opponent_shape_to_int(line.chars().next().unwrap()) + 1;
    let mine = my_shape_to_int(line.chars().last().unwrap()) + 1;

    match opponent - mine {
        0 => 3 + mine,
        -1 => 6 + mine,
        1 => 0 + mine,
        -2 => 0 + mine,
        2 => 6 + mine,
        _ => 0,
    }
}

fn score_second(line: &str) -> i32 {
    let opponent = opponent_shape_to_int(line.chars().next().unwrap());
    let round_end = line.chars().last().unwrap();

    match round_end {
        'X' => (opponent - 1).rem_euclid(3) + 1,
        'Y' => opponent + 3 + 1,
        'Z' => (opponent + 1).rem_euclid(3) + 6 + 1,
        _ => 0
    }
}

fn solve(filename: &str) {
    let content = fs::read_to_string(filename).unwrap();

    let first: i32 = content
        .trim_end()
        .split("\n")
        .map(|s| score(s))
        .sum();

    println!("A: {}", first);

    let second: i32 = content
        .trim_end()
        .split("\n")
        .map(|s| score_second(s))
        .sum();

    println!("B: {}", second);
}

fn main() {
    solve("input.txt")
}
