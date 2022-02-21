#![feature(stdin_forwarders)]

use day14::parse;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, (template, instructions)) = parse(&buf).expect("unable to parse input");
    let mut template: Vec<char> = template.chars().collect();
    
    for i in 1..=40 {
        let mut last: Option<char> = None;
        template = template.iter().tuple_windows().fold(Vec::new(), |mut acc, (a, b)| {
            acc.push(*a);
            match instructions.get(&(*a, *b)) {
                Some(insert) => acc.push(*insert),
                None => {},
            }
            last = Some(*b);
            acc
        });
        template.push(last.unwrap());
        // println!("polymer after {} iterations: {:?}", i, template);
        println!("iteration {} complete!", i);
        let mut counts: Vec<usize> = Vec::new();
        for (key, group) in &template.iter().sorted().group_by(|x| *x) {
            let count = group.count();
            counts.push(count);
            // println!("key: {:?}, group: {:?}", key, count);
        };

        match counts.iter().minmax() {
            MinMax(min, max) => println!("iteration {} len {} difference between max and min: {}, len - max - min: {}", i, template.len(), max - min, template.len() - max - min),
            _ => {},
        }
    }
    

}
