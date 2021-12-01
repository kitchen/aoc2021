use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).expect("please specify filename");
    println!("filename: {:?}", filename);

    let file = File::open(filename).expect("unable to open file");
    println!("opened file!");

    let lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok()?.parse().ok())
        .collect();

    let mut start = 0;
    let mut bigger = 0;
    let mut prev = None;
    while let (Some(a), Some(b), Some(c)) =
        (lines.get(start), lines.get(start + 1), lines.get(start + 2))
    {
        println!("prev: {:?} vs {} = {} + {} + {} ", prev, a + b + c, a, b, c);
        match prev {
            Some(prev) if a + b + c > prev => {
                bigger += 1;
            }
            _ => {}
        }
        start += 1;
        prev = Some(a + b + c);
    }
    println!("enbiggenings: {}", bigger);
}
