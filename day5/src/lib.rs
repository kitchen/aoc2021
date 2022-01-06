extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    sequence::separated_pair,
    multi::separated_list1,
    IResult,
};
use std::fmt;
use std::cmp;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)    
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)    
    }
}

impl Line {
    // returns the set of integer Points that make up this line
    pub fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        let xs: Vec<i32>;
        let ys: Vec<i32>;
        let len_x = (self.start.x - self.end.x).abs();
        let len_y = (self.start.y - self.end.y).abs();
        
        if self.start.x < self.end.x {
            xs = (self.start.x..=self.end.x).collect();
        } else if self.start.x == self.end.x {
            xs = vec!(self.start.x; len_y as usize + 1);
        } else {
            xs = (self.end.x..=self.start.x).rev().collect();
        }
        
        if self.start.y < self.end.y {
            ys = (self.start.y..=self.end.y).collect();
        } else if self.start.y == self.end.y {
            ys = vec!(self.start.y; len_x as usize + 1)
        } else {
            ys = (self.end.y..=self.start.y).rev().collect();
        }
        
        for (x, y) in xs.iter().zip(ys.iter()) {
            let point = Point { x: *x, y: *y };
            println!("adding: {}", point);
            points.insert(point);
             
        }
        points
    }
    
    pub fn is_vh(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

pub fn lines(input: &[u8]) -> IResult<&[u8], Vec<Line>> {
    separated_list1(tag("\n"), line)(input)
}

fn point(input: &[u8]) -> IResult<&[u8], Point> {
    let (rest, (x, y)) = separated_pair(
        i32,
        tag(","),
        i32,
    )(input)?;
    Ok((rest, Point {x: x, y: y}))
}

fn line(input: &[u8]) -> IResult<&[u8], Line> {
    // line looks like this:
    // 123,123 -> 123,123
    let (rest, (start, end)) = separated_pair(
        point,
        tag(" -> "),
        point,
    )(input)?;
    Ok((rest, Line { start: start, end: end}))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_point() {
        let case = "123,123".as_bytes();
        let (_, res) = point(case).unwrap();
        assert_eq!(res, Point { x: 123, y: 123 });
    }
    
    #[test]
    fn test_parse_line() {
        let case = "123,456 -> 42,24".as_bytes();
        let (_, res) = line(case).unwrap();
        assert_eq!(res, Line { start: Point { x: 123, y: 456 }, end: Point { x: 42, y: 24 }});
    }
    
    #[test]
    fn test_vert_horiz() {
        assert_eq!(Line { start: Point { x: 0, y: 0 }, end: Point { x: 0, y: 42 }}.is_vh(), true);
        assert_eq!(Line { start: Point { x: 0, y: 0 }, end: Point { x: 42, y: 0}}.is_vh(), true);
        assert_eq!(Line { start: Point { x: 0, y: 0 }, end: Point { x: 42, y: 42 }}.is_vh(), false);
    }
    
    #[test]
    fn test_points_vert() {
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 3 }
        };
        
        let res = line.points();
        let mut expected = HashSet::new();
        expected.insert(Point { x: 0, y: 0 });
        expected.insert(Point { x: 0, y: 1 });
        expected.insert(Point { x: 0, y: 2 });
        expected.insert(Point { x: 0, y: 3 });
        assert_eq!(res, expected);

      
    }
    
    #[test]
    fn test_points_horiz() {
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 3, y: 0 }
        };
        
        let res = line.points();
        let mut expected = HashSet::new();
        expected.insert(Point { x: 0, y: 0 });
        expected.insert(Point { x: 1, y: 0 });
        expected.insert(Point { x: 2, y: 0 });
        expected.insert(Point { x: 3, y: 0 });
        assert_eq!(res, expected);

        
    }
    
    #[test]
    fn test_points_diagonal_se() {
        let line = Line {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };
        
        let res = line.points();
        assert_eq!(res.len(), 3);
        
        let mut expected = HashSet::new();
        expected.insert(Point { x: 1, y: 1 });
        expected.insert(Point { x: 2, y: 2 });
        expected.insert(Point { x: 3, y: 3 });
        assert_eq!(res, expected);
    }
    
    #[test]
    fn test_points_diagonal_sw() {
        let line = Line {
            start: Point { x: 3, y: 3 },
            end: Point { x: 1, y: 5 },
        };
        
        let res = line.points();
        assert_eq!(res.len(), 3);
        
        let mut expected = HashSet::new();
        expected.insert(Point { x: 3, y: 3 });
        expected.insert(Point { x: 2, y: 4 });
        expected.insert(Point { x: 1, y: 5 });
        assert_eq!(res, expected);
    }
    
}