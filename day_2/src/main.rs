use std::error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let score: i32 = content
        .split("\n")
        .map(|line| Match::from_str(line).calculate_score())
        .sum();

    println!("Total: {}", score);
}

#[derive(Clone, Copy, Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

struct Match {
    mine: Throw,
    theirs: Throw,
}

impl Match {
    fn from_str(s: &str) -> Self {
        let throws: Vec<&str> = s.split(" ").collect();

        Match {
            theirs: match throws[0] {
                "A" => Throw::Rock,
                "B" => Throw::Paper,
                "C" => Throw::Scissors,
                _ => panic!("unknown opponent throw"),
            },
            mine: match throws[1] {
                "X" => Throw::Rock,
                "Y" => Throw::Paper,
                "Z" => Throw::Scissors,
                _ => panic!("unknown own throw"),
            },
        }
    }

    fn calculate_score(&self) -> i32 {
        match self.mine {
            Throw::Rock => {
                let throw_score = 1;
                throw_score
                    + match self.theirs {
                        Throw::Rock => 3,
                        Throw::Paper => 0,
                        Throw::Scissors => 6,
                    }
            }
            Throw::Paper => {
                let throw_score = 2;
                throw_score
                    + match self.theirs {
                        Throw::Rock => 6,
                        Throw::Paper => 3,
                        Throw::Scissors => 0,
                    }
            }
            Throw::Scissors => {
                let throw_score = 3;
                throw_score
                    + match self.theirs {
                        Throw::Rock => 0,
                        Throw::Paper => 6,
                        Throw::Scissors => 3,
                    }
            }
        }
    }
}
