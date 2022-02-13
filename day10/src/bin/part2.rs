#![feature(stdin_forwarders)]

use day10::parse;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let mut corrupted = false;
    let mut opens = Vec::new();
    let mut line = 1;
    let mut pos = 1;
    let mut scores = Vec::new();

    for char in buf.chars() {
        if char != '\n' && corrupted {
            continue;
        }
        
        match char {
            '\n' if !corrupted && opens.len() != 0 => {
                let line_score = complete(&opens);
                scores.push(line_score);
                println!("line {} is incomplete, score: {}, opens: {:?}", line, line_score, opens);
                line += 1;
                pos = 1;
                corrupted = false;
                opens.clear();
            },
            '\n' => {
                println!("new line found, opens length is {}", opens.len());
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
                corrupted = true;
            },
            ']' if opens.last() != Some(&'[') => {
                println!("line {}, pos {}: found ], was expecting the match to {:?}", line, pos, opens.last());
                corrupted = true;
            },
            '}' if opens.last() != Some(&'{') => {
                println!("line {}, pos {}: found }}, was expecting the match to {:?}", line, pos, opens.last());
                corrupted = true;
            },
            '>' if opens.last() != Some(&'<') => {
                println!("line {}, pos {}: found >, was expecting the match to {:?}", line, pos, opens.last());
                corrupted = true;
            },
            ')' | ']' | '}' | '>' => {
                opens.pop();
            },
            _ => panic!("totally invalid input: {}", char),

        }
        pos += 1;
    }
    

    let middle_index = scores.len() / 2;
    scores.sort();
    let middle_score = scores.get(middle_index).unwrap();
    println!("middle score: {}", middle_score);
}

fn complete(opens: &Vec<char>) -> u64{
    let mut score = 0;
    for open in opens.iter().rev() {
        score *= 5;
        match open {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => panic!("at the disco!"),
        }
    }
    score
}