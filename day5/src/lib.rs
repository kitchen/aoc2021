extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    sequence::separated_pair,
    multi::separated_list1,
    IResult,
};
use std::fmt;
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
        // "temporary" because I'm assuming the part2 will say "and also do this for diagonals based on X calculation"
        if self.is_vh() {
            if self.start.x == self.end.x {
                if self.start.y < self.end.y {
                    for y in (self.start.y)..=(self.end.y) {
                        let point = Point { x: self.start.x, y: y };
                        points.insert(point);
                    }
                } else {
                    for y in (self.end.y)..=(self.start.y) {
                        let point = Point { x: self.start.x, y: y };
                        points.insert(point);
                    }
                }
            } else {
                if self.start.x < self.end.x {
                    for x in (self.start.x)..=(self.end.x) {
                        let point = Point { x: x, y: self.start.y };
                        points.insert(point);
                    }
                } else {
                    for x in (self.end.x)..=(self.start.x) {
                        let point = Point { x: x, y: self.start.y };
                        points.insert(point);
                    }
                }
            }
        } else {
            unimplemented!();
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
    fn test_points_diagonal() {
    }
    
}