use std::{fs, str::FromStr};

pub fn advent_2() {
    let score1: i32 = fs::read_to_string("inputs/day2")
        .unwrap()
        .lines()
        .map(|x| Battle::try_from(x).unwrap().calc_points())
        .sum();

    let score2: i32 = fs::read_to_string("inputs/day2")
        .unwrap()
        .lines()
        .map(|x| FixedBattle::try_from(x).unwrap().as_battle().calc_points())
        .sum();

    println!("score 1: {}", score1);
    println!("score 2: {}", score2);
}

struct FixedBattle {
    opp: Choice,
    outcome: Outcome,
}

impl FixedBattle {
    fn as_battle(&self) -> Battle {
        let your_move = match self.opp {
            Choice::Rock => match self.outcome {
                Outcome::Win => Choice::Paper,
                Outcome::Draw => Choice::Rock,
                Outcome::Lose => Choice::Scissor,
            },
            Choice::Paper => match self.outcome {
                Outcome::Win => Choice::Scissor,
                Outcome::Draw => Choice::Paper,
                Outcome::Lose => Choice::Rock,
            },
            Choice::Scissor => match self.outcome {
                Outcome::Win => Choice::Rock,
                Outcome::Draw => Choice::Scissor,
                Outcome::Lose => Choice::Paper,
            },
        };

        Battle {
            you: your_move,
            opp: self.opp,
        }
    }
}

impl TryFrom<&str> for FixedBattle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let inputs: Vec<&str> = value.split(" ").collect();

        if inputs.len() != 2 {
            Err("should have two elements")
        } else {
            Ok(FixedBattle {
                opp: Choice::from_str(inputs[0])?,
                outcome: Outcome::from_str(inputs[1])?,
            })
        }
    }
}

struct Battle {
    you: Choice,
    opp: Choice,
}

impl Battle {
    pub fn calc_points(&self) -> i32 {
        let shape_points = match self.you {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        };

        let outcome = Self::resolve(self);

        let score_points = match outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        };

        score_points + shape_points
    }

    fn resolve(&self) -> Outcome {
        match self.you {
            Choice::Paper => match self.opp {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Draw,
                Choice::Scissor => Outcome::Lose,
            },
            Choice::Rock => match self.opp {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Lose,
                Choice::Scissor => Outcome::Win,
            },
            Choice::Scissor => match self.opp {
                Choice::Rock => Outcome::Lose,
                Choice::Paper => Outcome::Win,
                Choice::Scissor => Outcome::Draw,
            },
        }
    }
}

impl FromStr for Battle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: Vec<Choice> = s.split(" ").map(|x| Choice::from_str(x).unwrap()).collect();

        if choices.len() != 2 {
            return Err("should have two elements");
        }

        Ok(Battle {
            you: choices[1],
            opp: choices[0],
        })
    }
}

impl TryFrom<&str> for Battle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let choices: Vec<Choice> = value
            .split(" ")
            .map(|x| Choice::from_str(x).unwrap())
            .collect();

        if choices.len() != 2 {
            return Err("should have two elements");
        }

        Ok(Battle {
            you: choices[1],
            opp: choices[0],
        })
    }
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissor),
            _ => Err("not valid input"),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("not valid input"),
        }
    }
}
