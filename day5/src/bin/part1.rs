#![feature(stdin_forwarders)]

use day5::{Point, Line, lines};
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, lines) = lines(&buf).expect("unable to parse lines");
    
    let mut grid = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines.iter().filter(|line| {
        let foo = line.is_vh();
        if !foo {
            println!("{} is not vertical or horizontal", line);
        } else {
            println!("{} is vertical or horizontal!", line);
        }
        foo
    }) {
        for point in line.points() {
            let x = point.x;
            let y = point.y;
            let counter = grid.entry(point).or_insert(0);
            *counter += 1;
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }
    
    
    let mut grid_string: String = "".to_string();
    for y in 0..=max_y {
        for x in 0..=max_x {
            let point = Point { x: x, y: y};
            let mut count = 0;
            match grid.get(&point) {
                Some(value) => { count = *value },
                None => { count = 0 }
            }
            if count == 0 {
                grid_string.push_str(&".");
            } else {
                grid_string.push_str(&format!("{}", count).to_string());
            }
        }
        grid_string.push_str("\n");
    }
    println!("{}", grid_string);
    
    let morethanone = grid.iter().filter(|(point, count)| **count > 1).count();
    println!("grid: {:?}", grid);
    println!("answer: {}", morethanone);
 
}