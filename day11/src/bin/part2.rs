#![feature(stdin_forwarders)]

use day11::parse_map;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("unable to read to end or something");
 
    let mut map = parse_map(buf);
    

}