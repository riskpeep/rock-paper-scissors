//
// Rock, Paper, Scissors
//
// A game by Riskpeep

use std::fmt;
use std::io;
use std::str;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

enum RockPaperScissorsGuess {
    Rock,
    Paper,
    Scissors,
}

enum RockPaperScissorsCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutPaper,
}

enum RockPaperScissorsResult {
    Win(RockPaperScissorsCompare),
    Loss(RockPaperScissorsCompare),
    Tie(String),
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

impl Compare<RockPaperScissorsGuess, RockPaperScissorsResult> for RockPaperScissorsGuess{
    fn compare(&self, b: &RockPaperScissorsGuess) -> RockPaperScissorsResult {
        match self {
            RockPaperScissorsGuess::Rock => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Tie(self.to_string()),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::PaperCoversRock),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::RockCrushesScissors),
                }
            }
            RockPaperScissorsGuess::Paper => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::PaperCoversRock),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Tie(self.to_string()),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::ScissorsCutPaper),
                }
            }
            RockPaperScissorsGuess::Scissors => {
                match b {
                    RockPaperScissorsGuess::Rock    =>
                        RockPaperScissorsResult::Loss(RockPaperScissorsCompare::RockCrushesScissors),
                    RockPaperScissorsGuess::Paper   =>
                        RockPaperScissorsResult::Win(RockPaperScissorsCompare::ScissorsCutPaper),
                    RockPaperScissorsGuess::Scissors =>
                        RockPaperScissorsResult::Tie(self.to_string()),
                }
            }
        }
    }
}

impl fmt::Display for RockPaperScissorsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsResult::Win(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                }
            },
            RockPaperScissorsResult::Loss(result) => {
                match result {
                    RockPaperScissorsCompare::RockCrushesScissors => write!(f, "Rock crushes scissors"),
                    RockPaperScissorsCompare::PaperCoversRock => write!(f, "Paper covers rock"),
                    RockPaperScissorsCompare::ScissorsCutPaper => write!(f, "Scissors cut paper"),
                }
            },
            RockPaperScissorsResult::Tie(result) => write!(f, "Tie...{result}"),
        }
    }
}

#[derive(Debug)]
enum ParseRockPaperScissorsGuessError {
    Unknown(String),
}

impl str::FromStr for RockPaperScissorsGuess {
    type Err = ParseRockPaperScissorsGuessError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "rock"    => Ok(RockPaperScissorsGuess::Rock),
            "p" | "paper"   => Ok(RockPaperScissorsGuess::Paper),
            "s" | "scissors" => Ok(RockPaperScissorsGuess::Scissors),
            _   => Err(ParseRockPaperScissorsGuessError::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for RockPaperScissorsGuess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockPaperScissorsGuess::Rock    => write!(f, "Rock"),
            RockPaperScissorsGuess::Paper   => write!(f, "Paper"),
            RockPaperScissorsGuess::Scissors => write!(f, "Scissors"),
        }
    }
}

impl Distribution<RockPaperScissorsGuess> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RockPaperScissorsGuess {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => RockPaperScissorsGuess::Rock,
            1 => RockPaperScissorsGuess::Paper,
            2 => RockPaperScissorsGuess::Scissors,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Hello, Lets play Rock, Paper, Scissors!");

    let comp_move: RockPaperScissorsGuess = rand::thread_rng().gen();

    println!("Please select (r)ock, (p)aper, or (s)cissors:");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read move");

    let player_move: RockPaperScissorsGuess =
        player_move.trim().parse().expect("This is not a valid guess.");

    println!("You chose {player_move}");
    println!("I chose {comp_move}");

    let result: RockPaperScissorsResult = player_move.compare(&comp_move);

    println!("{}", result);
}
