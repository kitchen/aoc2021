#![feature(stdin_forwarders)]

use day6::fish;
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, mut feesh) = fish(&buf).expect("unable to parse feesh");
    println!("starting fish: {:?}", feesh);
    
    let iterations = 80;
    for i in 1..=iterations {
        let mut spawned = 0;
        for fish in &mut feesh {
            if *fish == 0 {
                *fish = 6;
                spawned += 1;
            } else {
                *fish -= 1;
            }
        }
        let new_feesh = vec![8; spawned];
        feesh.extend(new_feesh);
        println!("after iteration {}: {:?}", i, feesh);
    }
    
    println!("after {} days there are {} feesh", iterations, feesh.len());
    
}
