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
}
