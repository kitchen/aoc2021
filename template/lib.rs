extern crate nom;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::multi::separated_list1;
use nom::IResult;

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    separated_list1(tag(","), i32)(input)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_finds() {

    }
}