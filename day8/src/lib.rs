extern crate nom;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Digit {
    segments: HashSet<char>,
}

fn parse_digit(input: &str) -> IResult<&str, Digit> {
    let (rest, segment_string) = alpha1(input)?;
    
    let mut segments = HashSet::new();
    for x in segment_string.chars() {
        segments.insert(x);
    }
    Ok((rest, Digit { segments: segments} ))

}

pub fn parse_digits(input: &str) -> IResult<&str, Vec<Digit>> {
    separated_list1(char(' '), parse_digit)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, (Vec<Digit>, Vec<Digit>)> {
    separated_pair(parse_digits, tag(" | "), parse_digits)(input)
}

pub fn parse_lines(input: &str) -> IResult<&str, Vec<(Vec<Digit>, Vec<Digit>)>> {
    separated_list1(tag("\n"), parse_line)(input)
}

pub fn find_seven<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 3).unwrap()
}

pub fn find_one<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 2).unwrap()
}

pub fn find_four<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 4).unwrap()
}

pub fn find_eight<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 7).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_patterns() {
        let case = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let expected = vec!("acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab");
        
        let mut expected_one_segments = HashSet::new();
        expected_one_segments.insert('a');
        expected_one_segments.insert('b');
        let expected_one = Digit { segments: expected_one_segments };
        
        let (_rest, res) = parse_digits(case).unwrap();
        
        let res_one = &res[9];
        assert_eq!(res_one, &expected_one);
        
        let found_one = find_one(&res);
        assert_eq!(found_one, &expected_one);
        
        let mut expected_four_segments = HashSet::new();
        expected_four_segments.insert('a');
        expected_four_segments.insert('b');
        expected_four_segments.insert('e');
        expected_four_segments.insert('f');
        let expected_four = Digit { segments: expected_four_segments };
        
        let found_four = find_four(&res);
        assert_eq!(found_four, &expected_four);
        
        // seven
        let mut expected_seven_segments = HashSet::new();
        expected_seven_segments.insert('a');
        expected_seven_segments.insert('b');
        expected_seven_segments.insert('d');
        let expected_seven = Digit { segments: expected_seven_segments };
        
        let found_seven = find_seven(&res);
        assert_eq!(found_seven, &expected_seven);
        
        // eight
        let mut expected_eight_segments = HashSet::new();
        expected_eight_segments.insert('a');
        expected_eight_segments.insert('b');
        expected_eight_segments.insert('c');
        expected_eight_segments.insert('d');
        expected_eight_segments.insert('e');
        expected_eight_segments.insert('f');
        expected_eight_segments.insert('g');
        let expected_eight = Digit { segments: expected_eight_segments };
        
        let found_eight = find_eight(&res);
        assert_eq!(found_eight, &expected_eight);
        
        let input = "abc de | cba ed";
        let (_rest, (patterns, numbers)) = parse_line(input).unwrap();
        assert_eq!(patterns.len(), 2);
        assert_eq!(numbers.len(), 2);
    }
}