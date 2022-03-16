use std::collections::HashMap;

pub fn parse_map(input: String) -> (Vec<(i32, i32)>, HashMap<(i32, i32), usize>) {
    let mut map = HashMap::new();
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for char in input.chars() {
        if char == '\n' {
            y += 1;
            x = 0;
        } else {
            map.insert((x, y), char.to_digit(10).unwrap() as usize);
            coords.push((x, y));
            x += 1;
        }
    }
    (coords, map)
}

