#![feature(stdin_forwarders)]

use day8::*;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, lines) = parse_lines(&buf).unwrap();
    let mut total = 0;
    for (patterns, numbers) in lines {
        let one = find_one(&patterns);
        let two = find_two(&patterns);
        let three = find_three(&patterns);
        let four = find_four(&patterns);
        let five = find_five(&patterns);
        let six = find_six(&patterns);
        let seven = find_seven(&patterns);
        let eight = find_eight(&patterns);
        let nine = find_nine(&patterns);
        let zero = find_zero(&patterns);
        
        let value = numbers.iter().fold(0, |acc, x| {
            let acc = acc * 10;
            let mut foo = 0;
            if x == one {
                foo = 1
            } else if x == two {
                foo = 2
            } else if x == three {
                foo = 3
            } else if x == four {
                foo = 4
            } else if x == five {
                foo = 5
            } else if x == six {
                foo = 6
            } else if x == seven {
                foo = 7
            } else if x == eight {
                foo = 8
            } else if x == nine {
                foo = 9
            } else if x == zero {
                foo = 0
            }
            acc + foo
        });
        
        total += value
        
    }
    
    println!("total of all of the numbers: {}", total);


}
