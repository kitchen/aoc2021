#![feature(stdin_forwarders)]

use day4::game;
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, this_game) = game(&buf).expect("unable to parse game");
    println!("game: {:?}", this_game);

    for i in 5..(this_game.draws.len()) {
        let draws = &this_game.draws[0..i];
        match this_game.check_draws(draws) {
            Some(board) => {
                println!("winning draws: {:?}; winning board: {:?}", draws, board);
                let mut numbers = board.numbers.clone();
                println!("winning board's numbers: {:?}", numbers);
                
                numbers.retain(|&x| !draws.contains(&x));
                println!("remaining numbers: {:?}, sum: {:?}", numbers, numbers.iter().sum::<i32>());
                let lastdraw = this_game.draws[i-1];
                println!("last drawn number: {}, solution: {}", lastdraw, lastdraw * numbers.iter().sum::<i32>());
                
                break;
            }
            None => {}
        }
    }
}
