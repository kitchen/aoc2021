#![feature(stdin_forwarders)]

use day11::parse_map;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");
 
    let (coords, mut map) = parse_map(buf);
    let mut total_flashes = 0;
    
    // alg: go through and increment everything.
    // loop through and find all >= 10s flashing them and incrementing their neighbors. maintain a flashed list
    // repeat until there are no more zeroes that haven't flashed.
    // something tells me something about this won't scale in part 2
    for i in 1..=10000000 {
        println!("iteration: {}", i);
        for coord in &coords {
            map.entry(*coord).and_modify(|energy| *energy += 1);
        }
        
        let mut flashed: HashSet<&(i32, i32)> = HashSet::new();
        let mut flash_iter = 1;
        loop {
            let flashers: Vec<&(i32, i32)> = coords.iter().filter(|coord| {
                match map.get(coord) {
                    None => false,
                    Some(energy) => {
                        *energy >= 10 && !flashed.contains(coord)
                    },
                }
            }).collect();

            if flashers.len() == 0 {
                break;
            }

            for flasher in flashers {
                // find all 8 of its neighbers and increase them
                let x = flasher.0;
                let y = flasher.1;
                let neighbors = [
                    (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                    (x - 1, y    ),             (x + 1, y    ),
                    (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
                ];
                for neighbor in neighbors {
                    map.entry(neighbor).and_modify(|energy| *energy += 1);
                }

                flashed.insert(flasher);
                total_flashes += 1;
            }
            flash_iter += 1;

        }
        for coord in &flashed {
            map.entry(**coord).and_modify(|energy| *energy = 0);
        }
        
        if flashed.len() == coords.len() {
            println!("all octopuses flashed on step {}", i);
            break;
        }
    }

}

