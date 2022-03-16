#![feature(stdin_forwarders)]

use day15::parse;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");

    let (_, crabs) = parse(&buf).expect("unable to parse input");

}
