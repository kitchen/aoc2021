#![feature(stdin_forwarders)]

use day10::parse;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let mut score = 0;
    let mut corrupted = false;
    let mut opens = Vec::new();
    let mut line = 1;
    let mut pos = 1;

    for char in buf.chars() {
        if char != '\n' && corrupted {
            continue;
        }
        
        match char {
            '\n' => {
                line += 1;
                pos = 1;
                corrupted = false;
                opens.clear();
            },
            '{' | '[' | '(' | '<' => {
                opens.push(char)
            },
            ')' if opens.last() != Some(&'(') => {
                println!("line {}, pos {}: found ), was expecting the match to {:?}", line, pos, opens.last());
                score += 3;
                corrupted = true;
            },
            ']' if opens.last() != Some(&'[') => {
                println!("line {}, pos {}: found ], was expecting the match to {:?}", line, pos, opens.last());
                score += 57;
                corrupted = true;
            },
            '}' if opens.last() != Some(&'{') => {
                println!("line {}, pos {}: found }}, was expecting the match to {:?}", line, pos, opens.last());
                score += 1197;
                corrupted = true;
            },
            '>' if opens.last() != Some(&'<') => {
                println!("line {}, pos {}: found >, was expecting the match to {:?}", line, pos, opens.last());
                score += 25137;
                corrupted = true;
            },
            ')' | ']' | '}' | '>' => {
                opens.pop();
            },
            _ => panic!("totally invalid input: {}", char),

        }
        pos += 1;
    }
    
    println!("total syntax checker score is {}", score);

}
