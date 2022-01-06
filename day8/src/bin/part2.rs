#![feature(stdin_forwarders)]

use day8::parse;
use std::io::{self, Read};

fn main() {
    let mut buf = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("unable to read to end or something");

    let (_, crabs) = parse(&buf).expect("unable to parse input");

}
