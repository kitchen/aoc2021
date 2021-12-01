use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).expect("please specify filename");
    println!("filename: {:?}", filename);

    let file = File::open(filename).expect("unable to open file");
    println!("opened file!");

    let mut prev: Option<i32> = None;
    let mut bigger = 0;
    for line in io::BufReader::new(file).lines() {
        let current: i32 = line
            .expect("got a None line?")
            .parse()
            .expect("failed to parse line in file");
        println!("line: {}", current);
        match prev {
            Some(x) if current > x => {
                bigger += 1;
            }
            _ => {}
        }
        prev = Some(current);
    }
    println!("number of increments: {}", bigger);
}
