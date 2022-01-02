#![feature(stdin_forwarders)]

use day4::{game, Board};
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, this_game) = game(&buf).expect("unable to parse game");
    println!("game: {:?}", this_game);

    let mut longest: Option<Board> = None;
    let mut longest_len = 5;
    for board in this_game.boards {
        println!("board: {:?}", board);
        for i in 5..(this_game.draws.len()) {
            println!("checking draws len {}", i);
            let draws = &this_game.draws[0..i];
            if board.has_bingo(draws) {
                if i > longest_len {
                    println!("winner found!");
                    longest = Some(board);
                    longest_len = i;
                }
                break;
            }
        }
    }
    
    let draws = &this_game.draws[0..longest_len];
    println!("winning draws: {:?}; winning board: {:?}", draws, longest);
    let mut numbers = longest.unwrap().numbers.clone();
    println!("winning board's numbers: {:?}", numbers);
    
    numbers.retain(|&x| !draws.contains(&x));
    println!("remaining numbers: {:?}, sum: {:?}", numbers, numbers.iter().sum::<i32>());
    let lastdraw = this_game.draws[longest_len-1];
    println!("last drawn number: {}, solution: {}", lastdraw, lastdraw * numbers.iter().sum::<i32>());
                
}
