use itertools::Itertools;
use regex::Regex;
use std::{fs, str::FromStr};

pub fn advent_5() {
    let input = fs::read_to_string("inputs/day5").unwrap();


    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut layout = Layout::from_str(parts[0]).unwrap();

    parts[1]
        .lines()
        .map(|x| Move::from_str(x).unwrap())
        .for_each(|m| layout.apply_move(&m));

    let answer1 = layout
        .stacks
        .iter()
        .map(|stack| stack.to_owned().pop().unwrap())
        .join("");

    println!("answer 1: {}", answer1);

    let mut layout = Layout::from_str(parts[0]).unwrap();

    parts[1]
        .lines()
        .map(|x| Move::from_str(x).unwrap())
        .for_each(|m| layout.bulk_move(&m));

    let answer2 = layout
        .stacks
        .iter()
        .map(|stack| stack.to_owned().pop().unwrap())
        .join("");
    println!("answer 2: {}", answer2)
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\d+").unwrap();

        let caps = re.captures_iter(s).map(|x| x[0].to_string()).collect_vec();

        if caps.len() != 3 {
            Err("didn't find three numbers")
        } else {
            let nums = caps
                .iter()
                .map(|x| -> usize { x.parse().unwrap() })
                .collect_vec();

            Ok(Move {
                count: nums[0],
                from: nums[1],
                to: nums[2],
            })
        }
    }
}

#[derive(Debug)]
struct Layout {
    stacks: Vec<Vec<char>>,
}

impl Layout {
    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            let item = self.stacks[m.from - 1].pop().unwrap();
            self.stacks[m.to - 1].push(item);
        }
    }

    fn bulk_move(&mut self, m: &Move) {
        let mut batch: Vec<char> = Vec::new();

        for _ in 0..m.count {
            let item = self.stacks[m.from - 1].pop().unwrap();
            batch.push(item);
        }

        batch.reverse();
        self.stacks[m.to - 1].append(&mut batch);
    }
}

impl FromStr for Layout {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.split('\n').map(|x| x.to_owned()).collect_vec();

        let stack_count = l.pop().unwrap().split_whitespace().collect_vec().len();

        let mut stacks: Vec<Vec<char>> = (0..stack_count).map(|_| Vec::new()).collect_vec();

        l.iter().for_each(|x| {
            let contents = x.chars().chunks(4);

            for (idx, raw_item) in contents.into_iter().enumerate() {
                let item: String = raw_item.collect();
                let re = Regex::new(r"[A-Z]").unwrap();

                if let Some(caps) = re.captures(&item) {
                    stacks[idx].push(caps[0].chars().collect_vec()[0]);
                }
            }
        });

        let rev_stacks = stacks
            .iter()
            .map(|stack| stack.iter().rev().map(|c| c.to_owned()).collect_vec())
            .collect_vec();

        Ok(Layout { stacks: rev_stacks })
    }
}
