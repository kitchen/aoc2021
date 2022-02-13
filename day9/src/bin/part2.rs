#![feature(stdin_forwarders)]

use day9::parse_map;
use std::collections::{
    HashSet,
    HashMap,
};
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let map = parse_map(buf);
    let mut low_points = Vec::new();

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
            low_points.push((x, y));
            acc + value + 1
        } else {
            acc
        }
    });
    

    let mut basins: Vec<HashSet<(i32, i32)>> = low_points.iter().map(|&point| {
        let mut basin: HashSet<(i32, i32)> = HashSet::new();
        basin.insert((*point.0, *point.1));
        find_basin_neighbors(&map, &mut basin, (*point.0, *point.1));
        
        println!("low point {}, {} basin size {}", point.0, point.1, basin.len());
        basin
    }).collect();
    
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let result = basins.iter().take(3).fold(1, |acc, basin| basin.len() * acc);
    
    println!("product of largest 3 basin sizes: {}", result);

}

fn find_basin_neighbors(map: &HashMap<(i32, i32), i32>, basin: &mut HashSet<(i32, i32)>, coords: (i32, i32)) {
    let x = coords.0;
    let y = coords.1;
    let candidates = [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
    ];
    
    candidates.iter().for_each(|candidate| {
        if !basin.contains(candidate) {
            match map.get(candidate) {
                Some(value) if *value != 9 => {
                    basin.insert(*candidate);
                    find_basin_neighbors(&map, basin, *candidate);
                },
                _ => {},
            }
        }
            
    })
}
