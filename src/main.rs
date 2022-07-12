//
// Rock, Paper, Scissors
//
// A game by Riskpeep

use std::fmt;
use std::io;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

enum RockPaperScissorsGuess {
    Rock,
    Paper,
    Scissors,
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

    println!("Please select (r)ock, (p)aper, or (s)issors:");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read move");

    println!("You guessed: {player_move}");
    println!("I chose {comp_move}");
}
