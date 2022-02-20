extern crate nom;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::character::complete::alpha1;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::character::complete::anychar;
use nom::IResult;
use std::collections::HashMap;

pub fn parse(input: &str) -> IResult<&str, (&str, HashMap<(char, char), char>)> {
    separated_pair(alpha1, tag("\n\n"), instructions)(input)
}

fn instructions(input: &str) -> IResult<&str, HashMap<(char, char), char>> {
    let (rest, parsed_instructions) = separated_list1(tag("\n"), instruction)(input)?;
    
    let mut ret = HashMap::new();

    for (a, b, insert) in parsed_instructions {
        ret.insert((a, b), insert);
    }
    
    Ok((rest, ret))
}


fn instruction(input: &str) -> IResult<&str, (char, char, char)> {
    let (rest, (a, b, _, insert)) = tuple((anychar, anychar, tag(" -> "), anychar))(input)?;
    Ok((rest, (a, b, insert)))
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instruction() {
        let input = "AB -> C";

        let (_, foo) = instruction(input).unwrap();

    }
}