#![feature(stdin_forwarders)]

use day7::crabs;
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, mut crabs) = crabs(&buf).expect("unable to parse crabs");
    println!("starting crabs: {:?}", crabs);

    // ok, I think this is a "weighted mean" problem.
    // the thing I'm averaging is the distance of each crab from given point
    // weighted by the number of crabs at that point
    // that's how I'm going to approach it at first
    // I'll just iterate through min()..=max() and find the one with the ... smallest weighted mean?
    let min_pos = *crabs.iter().min().unwrap();
    let max_pos = *crabs.iter().max().unwrap();
    let num_crabs: i32 = crabs.len() as i32;
    
    let mut best_pos = 0;
    // there's a better way to do this but I'm lazy
    let mut best_fuel = 1231231231;
    
    for pos in min_pos..=max_pos {
        let total_fuel = crabs.iter().fold(0,|acc, crab| {
            let distance = (crab - pos).abs();
            (distance * (distance + 1)) / 2 + acc
        });
        println!("pos: {}, total fuel: {}", pos, total_fuel);
        if best_fuel > total_fuel {
            best_fuel = total_fuel;
            best_pos = pos;
        }
    }
    
    println!("best fuel is at pos {}: {}", best_pos, best_fuel);
}