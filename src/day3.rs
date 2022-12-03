use itertools::Itertools;
use std::{collections::HashSet, fs, str::FromStr};

pub fn advent_3() {
    let input = fs::read_to_string("inputs/day3").unwrap();

    let answer1: i32 = input
        .lines()
        .map(|x| Rucksack::from_str(x).unwrap().rucksack_priority())
        .sum();

    let answer2: i32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| {
            // println!("{}", c.into_iter().collect_vec());
            Group {
                rucksacks: c
                    .collect_vec()
                    .iter()
                    .map(|x| Rucksack::from_str(x).unwrap())
                    .collect(),
            }
            .group_priority()
        })
        .sum();

    println!("answer 1: {answer1}");
    println!("answer 2: {answer2}");
}

struct Group {
    // rucksacks: [Rucksack; 3],
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn group_priority(&self) -> i32 {
        let foo: HashSet<char> = self
            .rucksacks
            .iter()
            .map(|x| -> HashSet<char> {
                let concat = x.first_compartment.to_owned() + &x.second_compartment;
                concat.chars().collect()
            })
            .reduce(|acc, elem| -> HashSet<char> {
                acc.intersection(&elem)
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect()
            })
            .unwrap();

        if foo.len() != 1 {
            println!("{}", foo.len());
            panic!("oh no")
        } else {
            item_priority(&foo.into_iter().collect_vec()[0])
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl Rucksack {
    fn rucksack_priority(&self) -> i32 {
        let first_set: HashSet<char> = self.first_compartment.chars().collect();
        let second_set: HashSet<char> = self.second_compartment.chars().collect();

        let common: Vec<&char> = first_set.intersection(&second_set).collect();

        if common.len() != 1 {
            panic!("wtf man")
        }

        item_priority(common[0])
    }
}

impl FromStr for Rucksack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.len() / 2);

        Ok(Rucksack {
            first_compartment: first.to_owned(),
            second_compartment: second.to_owned(),
        })
    }
}

fn item_priority(item: &char) -> i32 {
    let chars = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

    let pos: i32 = chars
        .into_iter()
        .find_position(|c| c == &item.to_ascii_lowercase())
        .unwrap()
        .0
        .try_into()
        .unwrap();

    if item.is_ascii_lowercase() {
        pos + 1
    } else {
        pos + 27
    }
}
