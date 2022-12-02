use std::fs;

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
    opp: RockPaperScissor,
    outcome: Outcome,
}

impl FixedBattle {
    fn as_battle(&self) -> Battle {
        let your_move = match self.opp {
            RockPaperScissor::Rock => match self.outcome {
                Outcome::Win => RockPaperScissor::Paper,
                Outcome::Draw => RockPaperScissor::Rock,
                Outcome::Lose => RockPaperScissor::Scissor,
            },
            RockPaperScissor::Paper => match self.outcome {
                Outcome::Win => RockPaperScissor::Scissor,
                Outcome::Draw => RockPaperScissor::Paper,
                Outcome::Lose => RockPaperScissor::Rock,
            },
            RockPaperScissor::Scissor => match self.outcome {
                Outcome::Win => RockPaperScissor::Rock,
                Outcome::Draw => RockPaperScissor::Scissor,
                Outcome::Lose => RockPaperScissor::Paper,
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
            let opp = RockPaperScissor::try_from(inputs[0])?;
            let outcome = Outcome::try_from(inputs[1])?;

            Ok(FixedBattle { opp, outcome })
        }
    }
}

struct Battle {
    you: RockPaperScissor,
    opp: RockPaperScissor,
}

impl Battle {
    pub fn calc_points(&self) -> i32 {
        let shape_points = match self.you {
            RockPaperScissor::Rock => 1,
            RockPaperScissor::Paper => 2,
            RockPaperScissor::Scissor => 3,
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
            RockPaperScissor::Paper => match self.opp {
                RockPaperScissor::Rock => Outcome::Win,
                RockPaperScissor::Paper => Outcome::Draw,
                RockPaperScissor::Scissor => Outcome::Lose,
            },
            RockPaperScissor::Rock => match self.opp {
                RockPaperScissor::Rock => Outcome::Draw,
                RockPaperScissor::Paper => Outcome::Lose,
                RockPaperScissor::Scissor => Outcome::Win,
            },
            RockPaperScissor::Scissor => match self.opp {
                RockPaperScissor::Rock => Outcome::Lose,
                RockPaperScissor::Paper => Outcome::Win,
                RockPaperScissor::Scissor => Outcome::Draw,
            },
        }
    }
}

impl TryFrom<&str> for Battle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let choices: Vec<RockPaperScissor> = value
            .split(" ")
            .map(|x| RockPaperScissor::try_from(x).unwrap())
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
enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl TryFrom<&str> for RockPaperScissor {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(RockPaperScissor::Rock),
            "B" | "Y" => Ok(RockPaperScissor::Paper),
            "C" | "Z" => Ok(RockPaperScissor::Scissor),
            _ => Err("not valid input"),
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<&str> for Outcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("not valid input"),
        }
    }
}
