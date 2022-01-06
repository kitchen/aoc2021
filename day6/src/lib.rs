use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::multi::separated_list1;
use nom::IResult;

pub fn fish(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    separated_list1(tag(","), i32)(input)
}
