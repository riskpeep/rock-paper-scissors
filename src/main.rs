//
// Rock, Paper, Scissors
//
// A game by Riskpeep

use std::io;

fn main() {
    println!("Hello, Lets play Rock, Paper, Scissors!");

    println!("Please select (r)ock, (p)aper, or (s)issors:");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read move");

    println!("You guessed: {player_move}");
}
