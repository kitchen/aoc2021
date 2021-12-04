#![feature(stdin_forwarders)]

use day3::parse_line;
use std::io::{self};

fn main() {
    let report: Vec<i32> = io::stdin()
        .lines()
        .map(|line| day3::parse_line(line.expect("Error on line")))
        .collect();
    let gamma = gamma_rate(&report);
    println!("gamma: {}", gamma);
    let epsilon = epsilon_rate(&report);
    println!("epsilon: {}", epsilon);

    println!("product: {}", gamma * epsilon);
}

fn gamma_rate(report: &Vec<i32>) -> i32 {
    let report_size = report.len();
    let mut compare = 0b100000000000;
    let mut rate = 0;
    while compare != 0 {
        let num_ones = report.iter().filter(|x| *x & compare != 0).count();
        if num_ones > report_size - num_ones {
            rate += compare;
        }
        compare = compare >> 1;
    }
    rate
}

fn epsilon_rate(report: &Vec<i32>) -> i32 {
    let report_size = report.len();
    let mut compare = 0b100000000000;
    let mut rate = 0;
    while compare != 0 {
        let num_ones = report.iter().filter(|x| *x & compare != 0).count();
        if num_ones < report_size - num_ones {
            rate += compare;
        }
        compare = compare >> 1;
    }
    rate
}
