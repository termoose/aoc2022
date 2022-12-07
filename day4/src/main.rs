use std::{fs, ops::Range};

struct Interval {
    range: Range<i32>,
}

impl Interval {
    fn new(start: i32, end: i32) -> Interval {
        Interval{ range: start..(end+1) }
    }

    fn from_str(form: &str) -> Interval {
        let nums: Vec<&str> = form.split("-").collect();
        let start_str: &str = nums.first().unwrap();
        let end_str: &str = nums.last().unwrap();
        let start = start_str.parse::<i32>().unwrap();
        let end = end_str.parse::<i32>().unwrap();

        Interval::new(start, end)
    }

    fn contains(&self, other: &Interval) -> bool {
        match self.range.len() > other.range.len() {
            true => other.range.clone().all(|n| {
                self.range.contains(&n)
            }),
            false => self.range.clone().all(|n| {
                other.range.contains(&n)
            })
        }
    }

    fn overlap(&self, other: &Interval) -> bool {
        other.range.clone().any(|n| {
            self.range.contains(&n)
        })
    }
}

type IntervalPairs = (Interval, Interval);

fn into_interval_pairs(line: &str) -> IntervalPairs {
    let pairs: Vec<&str> = line.split(",").collect();
    let first = pairs.first().unwrap();
    let second = pairs.last().unwrap();

    (Interval::from_str(first), Interval::from_str(second))
}


fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let total_contains: usize = content
        .trim_end()
        .split("\n")
        .map(|l| into_interval_pairs(l))
        .filter(|(a,b)| a.contains(&b))
        .count();

    let total_overlaps: usize = content
        .trim_end()
        .split("\n")
        .map(|l| into_interval_pairs(l))
        .filter(|(a, b)| a.overlap(&b))
        .count();

    println!("A: {}", total_contains);
    println!("B: {}", total_overlaps);

}
