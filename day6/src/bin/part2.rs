#![feature(stdin_forwarders)]

use day6::fish;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, mut feesh) = fish(&buf).expect("unable to parse feesh");
    println!("starting fish: {:?}", feesh);
    
    let iterations = 40;
    
    let mut feesh_counts: HashMap<u64, u64> = feesh.iter().fold(HashMap::new(), |mut counts, fish| {
        counts.entry((*fish).try_into().unwrap())
            .and_modify(|x| *x += 1 )
            .or_insert(1);
        counts
    });
    
    println!("feesh counts: {:?}", feesh_counts);
    
    let days = 256;
    for day in 1..=days {
        let mut new_counts = HashMap::new();
        for days_left in (0..=8).rev() {
            let count = *feesh_counts.entry(days_left).or_insert(0);
            
            
            if days_left == 0 {
                let six = new_counts.entry(6).or_insert(0);
                *six += count;
                new_counts.insert(8, count);
            } else {
                new_counts.insert(days_left - 1, count);
            }
        }
        feesh_counts = new_counts;
        let total_feesh: u64 = feesh_counts.iter().fold(0, |acc, (_, count)| acc + count );
        println!("total feesh after {} days: {}; counts: {:?}", day, total_feesh, feesh_counts);
    }
    
    
    
    
}
