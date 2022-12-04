use std::{collections::HashSet, fs, str::FromStr};

use itertools::Itertools;

pub fn advent_4() {
    let input = fs::read_to_string("inputs/day4").unwrap();

    let overlap_count = input
        .lines()
        .map(|x| Pair::from_str(x).unwrap())
        .filter(|pair| pair.has_full_overlap())
        .count();

    println!("answer 1: {overlap_count}");

    let answer2 = input
        .lines()
        .map(|x| Pair::from_str(x).unwrap())
        .filter(|pair| pair.has_any_overlap())
        .count();

    println!("answer 2: {answer2}");
}

struct Section {
    start: i32,
    end: i32,
}

impl Section {
    fn is_contained_in(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn has_overlap(&self, other: &Self) -> bool {
        let self_ids: HashSet<i32> = HashSet::from_iter(self.sector_ids());
        let other_ids: HashSet<i32> = HashSet::from_iter(other.sector_ids());

        self_ids.intersection(&other_ids).count() > 0
    }

    fn sector_ids(&self) -> Vec<i32> {
        (self.start..self.end + 1).collect_vec()
    }
}

impl FromStr for Section {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split('-')
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect_tuple()
            .unwrap();

        Ok(Section { start, end })
    }
}

struct Pair {
    sections: [Section; 2],
}

impl Pair {
    fn has_full_overlap(&self) -> bool {
        self.sections[0].is_contained_in(&self.sections[1])
            || self.sections[1].is_contained_in(&self.sections[0])
    }

    fn has_any_overlap(&self) -> bool {
        self.sections[0].has_overlap(&self.sections[1])
            || self.sections[1].has_overlap(&self.sections[0])
    }
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s
            .split(',')
            .map(|x| Section::from_str(x).unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Pair { sections: [p1, p2] })
    }
}
