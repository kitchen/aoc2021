use std::env;
use std::fs::File;
use std::io::{self, BufRead};

struct Coordinates {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args.get(1).expect("please specify filename");
    println!("filename: {:?}", filename);

    let file = File::open(filename).expect("unable to open file");
    println!("opened file!");

    let mut cur = Coordinates { x: 0, y: 0 };

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("line is None?");
        let line_parts: Vec<&str> = line.split(" ").collect();
        let direction = line_parts
            .get(0)
            .expect("unable to parse direction from line");
        let distance = line_parts
            .get(1)
            .expect("unable to parse distance from line");
        let distance_num: i32 = distance.parse().expect("unable to parse distance to int");

        cur = match *direction {
            "forward" => forward(distance_num, cur),
            "up" => up(distance_num, cur),
            "down" => down(distance_num, cur),
            _ => {
                panic!("invalid direction found")
            }
        };

        println!("line: {}, cur: {}, {}", line, cur.x, cur.y);
    }

    println!(
        "final position: {}, {}; product: {}",
        cur.x,
        cur.y,
        cur.x * cur.y
    );
}

fn up(up: i32, cur: Coordinates) -> Coordinates {
    Coordinates {
        y: cur.y - up,
        ..cur
    }
}
fn down(down: i32, cur: Coordinates) -> Coordinates {
    Coordinates {
        y: cur.y + down,
        ..cur
    }
}
fn forward(forward: i32, cur: Coordinates) -> Coordinates {
    Coordinates {
        x: cur.x + forward,
        ..cur
    }
}
