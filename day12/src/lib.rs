extern crate nom;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::character::complete::alpha1;
use nom::IResult;
use std::collections::HashMap;
use std::collections::HashSet;

// a-b
// b-c
// c-a
pub fn parse(input: &str) -> IResult<&str, HashMap<&str, HashSet<&str>>> {
    let mut nodes: HashMap<&str, HashSet<&str>> = HashMap::new();
    let (rest, lines) = separated_list1(tag("\n"), line)(input)?;
    for (a, b) in lines {
        nodes.entry(a).and_modify(|neighbors| { neighbors.insert(b); }).or_insert_with(|| { let mut neighbors = HashSet::new(); neighbors.insert(b); neighbors });
        nodes.entry(b).and_modify(|neighbors| { neighbors.insert(a); }).or_insert_with(|| { let mut neighbors = HashSet::new(); neighbors.insert(a); neighbors });
    }
    
    Ok((rest, nodes))
}

pub fn line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-"), alpha1)(input)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_line() {
        let input = "start-ab";
        let expected = ("start", "ab");
        let (_, actual) = line(input).unwrap();
        assert_eq!(actual, expected);

    }
    
    #[test]
    fn test_parse() {
        let input = "start-ab\nstart-ba\nba-ab\nab-end";
        let (_, lines) = parse(input).unwrap();
        assert!(lines.contains_key("start"));
        assert!(lines.get("start").unwrap().contains("ba"));
    }
}