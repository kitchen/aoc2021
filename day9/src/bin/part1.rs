#![feature(stdin_forwarders)]

use day9::parse_map;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let map = parse_map(buf);
    let foo = map.iter().fold(0, |acc, ((x, y), value)| {
        let candidates = [
            (*x - 1, *y),
            (*x + 1, *y),
            (*x, *y - 1),
            (*x, *y + 1),
        ];
        
        let bar = candidates.iter().fold(0, |x, candidate| {
            match map.get(candidate) {
                Some(candidate_value) if value >= candidate_value => { x + 1 },
                _ => { x },
            }
        });
        
        if bar == 0 {
            println!("{}, {} is a low point, risk is: {}", x, y, value + 1);
            acc + value + 1
        } else {
            acc
        }
    });
    
    println!("sum of low points: {}", foo);


}
