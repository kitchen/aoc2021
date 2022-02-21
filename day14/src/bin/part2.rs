#![feature(stdin_forwarders)]

use day14::parse;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, (template, instructions)) = parse(&buf).expect("unable to parse input");
    
    let mut pairs_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in template.chars().tuple_windows() {
        pairs_counts.entry(pair).and_modify(|x| *x += 1).or_insert(1);
    }
    let last = template.chars().last().unwrap();
    
    for i in 1..=40 {
        let mut new_pairs_counts: HashMap<(char, char), usize> = HashMap::new();
        let mut first_counts: HashMap<char, usize> = HashMap::new();
        for ((a, b), count) in pairs_counts {
            match instructions.get(&(a, b)) {
                Some(insert) => {
                    new_pairs_counts.entry((a, *insert)).and_modify(|x| *x += count).or_insert(count);
                    first_counts.entry(a).and_modify(|x| *x += count).or_insert(count);
                    new_pairs_counts.entry((*insert, b)).and_modify(|x| *x += count).or_insert(count);
                    first_counts.entry(*insert).and_modify(|x| *x += count).or_insert(count);
                },
                None => { new_pairs_counts.entry((a, b)).and_modify(|x| *x += count).or_insert(count); },
            }
            

        }
        match first_counts.values().minmax() {
            MinMax(min, max) => { println!("max ({}) - min ({}) = {}", max, min, max - min); },
            _ => {},
            
        }
        pairs_counts = new_pairs_counts;

        
    }
    println!("pairs counts: {:?}", pairs_counts);
}
