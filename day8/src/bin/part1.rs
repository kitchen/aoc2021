#![feature(stdin_forwarders)]

use day8::{
    parse_lines,
    find_one,
    find_four,
    find_seven,
    find_eight,
};
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, lines) = parse_lines(&buf).unwrap();
    let mut count = 0;
    for (patterns, numbers) in lines {
        let one = find_one(&patterns);
        let four = find_four(&patterns);
        let seven = find_seven(&patterns);
        let eight = find_eight(&patterns);
        
        for number in numbers {
            if number == *one || number == *four || number == *seven || number == *eight {
                count += 1;
            }
        }
    }
    println!("found 1, 4, 7, 8 this many times: {}", count);
}
