use std::fs;
use itertools::Itertools;

pub fn advent_1() {
    let counts: Vec<i32> = fs::read_to_string("inputs/day1")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .split(|s| s.is_empty())
        .map(|c| {
            c.iter()
                .map(|s| s.parse::<i32>().expect("fuck you too"))
                .sum::<i32>()
        })
        .collect();

    let max = counts.iter().max().unwrap();

    let top_3: i32 = counts.iter().sorted().rev().take(3).sum();

    println!("max: {}", max);
    println!("top 3 sum: {}", top_3);
}
