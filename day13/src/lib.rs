extern crate nom;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum Fold {
    X(i32),
    Y(i32),
}

pub fn parse(input: &str) -> IResult<&str, (Vec<(i32,i32)>, Vec<Fold>)> {
    let (rest, coords) = terminated(separated_list1(tag("\n"), coord_line), tag("\n\n"))(input)?;
    
    let (rest, folds) = separated_list1(tag("\n"), fold_line)(rest)?;
    
    

    Ok((rest, (coords, folds)))
}

fn coord_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(","), i32)(input)
}

fn fold_line(input: &str) -> IResult<&str, Fold> {
    let (res, (_, (direction, line))) = tuple((tag("fold along "), separated_pair(alpha1, tag("="), i32)))(input)?;
    
    match direction {
        "x" => Ok((res, Fold::X(line))),
        "y" => Ok((res, Fold::Y(line))),
        _ => panic!("bad fold direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse() {
        let input = "4,11\n6,0\n\nfold along y=3\nfold along x=5";
        let (rest, (coords, folds)) = parse(input).unwrap();

        let expected_coords = vec![(4,11),(6,0)];
        assert_eq!(coords, expected_coords);
        
        let expected_folds = vec![Fold::Y(3), Fold::X(5)];
        assert_eq!(folds, expected_folds);

    }
    
    #[test]
    fn test_fold_line() {
        let input = "fold along x=5";
        let (_, fold) = fold_line(input).unwrap();
        assert_eq!(Fold::X(5), fold);
    }
}