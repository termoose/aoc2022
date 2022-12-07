use std::{fs, collections::HashSet};

type Rugsacks = Vec<(HashSet<i32>, HashSet<i32>)>;

fn to_priority(item: char) -> i32 {
    match item.is_uppercase() {
        true => item as i32 - 'A' as i32 + 27,
        false => item as i32 - 'a' as i32 + 1
    }
}

fn to_priorities(compartment: &str) -> HashSet<i32> {
    compartment.chars().map(|i| to_priority(i)).collect()
}

fn input_to_sacks(filename: &str) -> Rugsacks {
    fs::read_to_string(filename).unwrap()
        .trim_end()
        .split("\n")
        .map(|rugsack| {
            let half = rugsack.len() / 2;

            (to_priorities(&rugsack[..half]),
             to_priorities(&rugsack[half..]))
        })
        .collect()
}

fn intersect_rugsacks(rugsacks: Rugsacks) -> Vec<i32> {
    rugsacks.into_iter().map(|(c1, c2)| {
        c1.intersection(&c2).sum::<i32>()
    }).collect()
}

fn main() {
    let lol = input_to_sacks("input.txt");
    let ok: i32 = intersect_rugsacks(lol).into_iter().sum();

    println!("{:?}", ok);
}
