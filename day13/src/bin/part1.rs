#![feature(stdin_forwarders)]

use day13::parse;
use day13::Fold;
use itertools::Itertools;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, (coords, folds)) = parse(&buf).expect("unable to parse input");
    
    let final_positions = do_folds(coords, folds.as_slice());

    let (max_x, max_y) = final_positions.iter().fold((0,0), |(acc_x, acc_y), (x, y)| {
        (if *x > acc_x { *x } else { acc_x }, if *y > acc_y { *y } else { acc_y })
    });
    
    for y in 0..=max_y {
        for x in 0..=max_x {
            if final_positions.contains(&(x, y)) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }


}

fn do_folds(coords: Vec<(i32, i32)>, folds: &[Fold]) -> Vec<(i32, i32)> {
    let new_coords: Vec<(i32, i32)> = coords.iter().map(|(x, y)| {
        match folds.first() {
            Some(Fold::X(line)) if line < x => {
                (line - (*x - line), *y)
            },
            Some(Fold::Y(line)) if line < y => {
                (*x, line - (*y - line))
            },
            Some(thing) => (*x, *y),
            _ => panic!("eek!"),
        }
    }).unique().collect();
    
    println!("number of dots: {}", new_coords.len());
    
    
    if folds.len() == 1 {
        new_coords
    } else {
        do_folds(new_coords, &folds[1..])
    }
    
}
