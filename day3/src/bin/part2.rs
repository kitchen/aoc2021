#![feature(stdin_forwarders)]

use day3::parse_line;
use std::io::{self};

fn main() {
    let report: Vec<i32> = io::stdin()
        .lines()
        .map(|line| day3::parse_line(line.expect("Error on line")))
        .collect();
    let oxygen = oxygen_rating(report.clone(), 0b100000000000);
    println!("oxygen rating: {}", oxygen);
    let co2 = co2_rating(report, 0b100000000000);
    println!("co2 rating: {}", co2);
    println!("product: {}", co2 * oxygen);
}

fn oxygen_rating(input: Vec<i32>, pos: i32) -> i32 {
    println!("got input: {:?}", input);
    match input.len() {
        0 => panic!("ran out of numbers before reaching 1"),
        1 => input[0],
        _ => {
            let (ones, zeroes): (Vec<i32>, Vec<i32>) = input.iter().partition(|a| *a & pos != 0);
            let nextpos = pos >> 1;
            if ones.len() >= zeroes.len() {
                println!("more ones");
                oxygen_rating(ones, nextpos)
            } else {
                println!("more zeroes");
                oxygen_rating(zeroes, nextpos)
            }
        }
    }
}

fn co2_rating(input: Vec<i32>, pos: i32) -> i32 {
    println!("got input: {:?}", input);
    match input.len() {
        0 => panic!("ran out of numbers before reaching 1"),
        1 => input[0],
        _ => {
            let (ones, zeroes): (Vec<i32>, Vec<i32>) = input.iter().partition(|a| *a & pos != 0);
            let nextpos = pos >> 1;
            if ones.len() < zeroes.len() {
                println!("less ones");
                co2_rating(ones, nextpos)
            } else {
                println!("less zeroes");
                co2_rating(zeroes, nextpos)
            }
        }
    }
}
