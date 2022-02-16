use std::collections::HashMap;

pub fn parse_map(input: String) -> (Vec<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut map = HashMap::new();
    let mut coords: Vec<(i32, i32)> = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for char in input.chars() {
        if char == '\n' {
            y += 1;
            x = 0;
        } else {
            map.insert((x, y), char.to_digit(10).unwrap() as i32);
            coords.push((x, y));
            x += 1;
        }
    }
    (coords, map)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let input = "123\n456".to_string();
        let mut expected = HashMap::new();
        expected.insert((0,0), 1);
        expected.insert((1,0), 2);
        expected.insert((2,0), 3);
        expected.insert((0,1), 4);
        expected.insert((1,1), 5);
        expected.insert((2,1), 6);
        let mut actual = parse_map(input);
        
        assert_eq!(actual, expected);
        
        for (k, v) in &mut actual {
            *v += 1;
        }
        
        assert_eq!(actual.get(&(0,0)), Some(&2));
    }
}

