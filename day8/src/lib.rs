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

pub fn find_one<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 2).unwrap()
}

pub fn find_two<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let three = find_three(patterns);
    let four = find_four(patterns);
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 5 && pattern != &three);
    candidates.find(|pattern| pattern.segments.difference(&four.segments).count() == 3).unwrap()
}

pub fn find_three<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let one = find_one(patterns);
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 5);
    candidates.find(|pattern| pattern.segments.difference(&one.segments).count() == 3).unwrap()
}

pub fn find_four<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 4).unwrap()
}

pub fn find_five<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let three = find_three(patterns);
    let four = find_four(patterns);
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 5 && pattern != &three);
    candidates.find(|pattern| pattern.segments.difference(&four.segments).count() == 2).unwrap()
}

pub fn find_six<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let one = find_one(patterns);
    let five = find_five(patterns);
    let upper_right = one.segments.difference(&five.segments).last().unwrap();
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 6);
    candidates.find(|pattern| !pattern.segments.contains(upper_right)).unwrap()
}

pub fn find_seven<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 3).unwrap()
}

pub fn find_eight<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    patterns.iter().find(|pattern| pattern.segments.len() == 7).unwrap()
}

pub fn find_nine<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let four = find_four(patterns);
    let six = find_six(patterns);
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 6 && pattern != &six);
    candidates.find(|pattern| pattern.segments.difference(&four.segments).count() == 2).unwrap()
}

pub fn find_zero<'a>(patterns: &'a Vec<Digit>) -> &'a Digit {
    let four = find_four(patterns);
    let six = find_six(patterns);
    let mut candidates = patterns.iter().filter(|pattern| pattern.segments.len() == 6 && pattern != &six);
    candidates.find(|pattern| pattern.segments.difference(&four.segments).count() == 3).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_finds() {
        let case = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let one_string = "ab";
        let (_, one_digit) = parse_digit(one_string).unwrap();
        let two_string = "gcdfa";
        let (_, two_digit) = parse_digit(two_string).unwrap();
        let three_string = "fbcad";
        let (_, three_digit) = parse_digit(three_string).unwrap();
        let four_string = "eafb";
        let (_, four_digit) = parse_digit(four_string).unwrap();
        let five_string = "cdfbe";
        let (_, five_digit) = parse_digit(five_string).unwrap();
        let six_string = "cdfgeb";
        let (_, six_digit) = parse_digit(six_string).unwrap();
        let seven_string = "dab";
        let (_, seven_digit) = parse_digit(seven_string).unwrap();
        let eight_string = "acedgfb";
        let (_, eight_digit) = parse_digit(eight_string).unwrap();
        let nine_string = "cefabd";
        let (_, nine_digit) = parse_digit(nine_string).unwrap();
        let zero_string = "cagedb";
        let (_, zero_digit) = parse_digit(zero_string).unwrap();
        
        let (_rest, digits) = parse_digits(case).unwrap();
        
        let found_one = find_one(&digits);
        assert_eq!(found_one, &one_digit);
        let found_two = find_two(&digits);
        assert_eq!(found_two, &two_digit);
        let found_three = find_three(&digits);
        assert_eq!(found_three, &three_digit);
        let found_four = find_four(&digits);
        assert_eq!(found_four, &four_digit);
        let found_five = find_five(&digits);
        assert_eq!(found_five, &five_digit);
        let found_six = find_six(&digits);
        assert_eq!(found_six, &six_digit);
        let found_seven = find_seven(&digits);
        assert_eq!(found_seven, &seven_digit);
        let found_eight = find_eight(&digits);
        assert_eq!(found_eight, &eight_digit);
        let found_nine = find_nine(&digits);
        assert_eq!(found_nine, &nine_digit);
        let found_zero = find_zero(&digits);
        assert_eq!(found_zero, &zero_digit);
        
    }
    
}