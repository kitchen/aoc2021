extern crate nom;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    multi::{count, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub numbers: Vec<i32>,
}

impl Board {
    fn has_bingo(&self, draws: &[i32]) -> bool {
        // obviously if there are less than 5 draws there can't be a bingo yet
        if draws.len() < 5 {
            return false;
        }

        let draws_set: HashSet<&i32> = HashSet::from_iter(draws);
        println!("draws_set: {:?}", draws_set);

        // really should precompute these
        for row_num in 0..=4 {
            println!("row number: {}", row_num);
            let row = &self.numbers[(row_num * 5)..(row_num * 5 + 5)];
            println!("row: {:?}", row);
            let row_set = HashSet::from_iter(row);
            if draws_set.is_superset(&row_set) {
                return true;
            }
        }

        // really should precompute these
        for col_num in 0..=4 {
            println!("col number: {}", col_num);
            let col = &[
                self.numbers[col_num],
                self.numbers[col_num + 5],
                self.numbers[col_num + 10],
                self.numbers[col_num + 15],
                self.numbers[col_num + 20],
            ];
            let col_set: HashSet<&i32> = HashSet::from_iter(col);
            println!("col_set: {:?}", col_set);
            if draws_set.is_superset(&col_set) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub draws: Vec<i32>,
    pub boards: Vec<Board>,
}

impl Game {
    // if there is a winning board, return it
    pub fn check_draws(&self, draws: &[i32]) -> Option<&Board> {
        for board in &self.boards {
            if board.has_bingo(draws) {
                return Some(board);
            }
        }
        None
    }
}

fn draws(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    separated_list1(tag(","), i32)(input)
}

fn board_line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    separated_list1(tag(" "), preceded(multispace0, i32))(input)
}

fn board(input: &[u8]) -> IResult<&[u8], Board> {
    let (rest, rows) = count(terminated(board_line, tag("\n")), 5)(input)?;
    let mut numbers = Vec::new();
    for mut row in rows {
        numbers.append(&mut row);
    }
    Ok((rest, Board { numbers: numbers }))
}

fn boards(input: &[u8]) -> IResult<&[u8], Vec<Board>> {
    separated_list1(tag("\n"), board)(input)
}

pub fn game(input: &[u8]) -> IResult<&[u8], Game> {
    let (rest, draws_part) = terminated(draws, tag("\n\n"))(input)?;
    let (rest, boards_part) = boards(rest)?;

    Ok((
        rest,
        Game {
            draws: draws_part,
            boards: boards_part,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_draws() {
        let case = "1,2,3,42,18,32\n".as_bytes();
        let res = draws(case);
        let rest = "\n".as_bytes();
        assert_eq!(Ok((rest, vec![1, 2, 3, 42, 18, 32])), res);
    }

    #[test]
    fn test_board_line() {
        let case = "1 16 31 46 61\n".as_bytes();
        let res = board_line(case);
        let rest: &[u8] = "\n".as_bytes();
        assert_eq!(Ok((rest, vec![1, 16, 31, 46, 61])), res);
    }

    #[test]
    fn test_board() {
        let case =
            " 1 16 31 46 61\n 2 17 32 47 62\n 3 18 33 48 63\n 4 19 34 49 64\n 5 20 35 50 65\n\n"
                .as_bytes();
        let res = board(case);
        let rest = "\n".as_bytes();
        let expected = Board {
            numbers: vec![
                1, 16, 31, 46, 61, 2, 17, 32, 47, 62, 3, 18, 33, 48, 63, 4, 19, 34, 49, 64, 5, 20,
                35, 50, 65,
            ],
        };
        assert_eq!(Ok((rest, expected)), res);
    }

    #[test]
    fn test_boards() {
        let case = concat!(
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n",
            "\n",
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n",
            "\n",
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n"
        )
        .as_bytes();
        let res = boards(case);
        let rest = "".as_bytes();
        let expected_numbers = vec![
            1, 16, 31, 46, 61, 2, 17, 32, 47, 62, 3, 18, 33, 48, 63, 4, 19, 34, 49, 64, 5, 20, 35,
            50, 65,
        ];
        let expected_each = Board {
            numbers: expected_numbers.clone(),
        };
        let expected = vec![
            expected_each.clone(),
            expected_each.clone(),
            expected_each.clone(),
        ];
        assert_eq!(Ok((rest, expected)), res);
    }

    #[test]
    fn test_game() {
        let case = concat!(
            "1,2,3,42,46,8,12\n",
            "\n",
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n",
            "\n",
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n",
            "\n",
            "1 16 31 46 61\n2 17 32 47 62\n3 18 33 48 63\n4 19 34 49 64\n5 20 35 50 65\n"
        )
        .as_bytes();
        let (actual_rest, actual_game) = game(case).unwrap();

        let expected_rest = "".as_bytes();

        assert_eq!(7, actual_game.draws.len());
        assert_eq!(3, actual_game.boards.len());
        assert_eq!(expected_rest, actual_rest);
    }

    #[test]
    fn test_more_boards() {
        let case: &[u8] = &[
            49, 32, 49, 54, 32, 51, 49, 32, 52, 54, 32, 54, 49, 10, 50, 32, 49, 55, 32, 51, 50, 32,
            52, 55, 32, 54, 50, 10, 51, 32, 49, 56, 32, 51, 51, 32, 52, 56, 32, 54, 51, 10, 52, 32,
            49, 57, 32, 51, 52, 32, 52, 57, 32, 54, 52, 10, 53, 32, 50, 48, 32, 51, 53, 32, 53, 48,
            32, 54, 53, 10, 10, 49, 32, 49, 54, 32, 51, 49, 32, 52, 54, 32, 54, 49, 10, 50, 32, 49,
            55, 32, 51, 50, 32, 52, 55, 32, 54, 50, 10, 51, 32, 49, 56, 32, 51, 51, 32, 52, 56, 32,
            54, 51, 10, 52, 32, 49, 57, 32, 51, 52, 32, 52, 57, 32, 54, 52, 10, 53, 32, 50, 48, 32,
            51, 53, 32, 53, 48, 32, 54, 53, 10, 10, 49, 32, 49, 54, 32, 51, 49, 32, 52, 54, 32, 54,
            49, 10, 50, 32, 49, 55, 32, 51, 50, 32, 52, 55, 32, 54, 50, 10, 51, 32, 49, 56, 32, 51,
            51, 32, 52, 56, 32, 54, 51, 10, 52, 32, 49, 57, 32, 51, 52, 32, 52, 57, 32, 54, 52, 10,
            53, 32, 50, 48, 32, 51, 53, 32, 53, 48, 32, 54, 53, 10,
        ];
        let (actual_rest, actual_boards) = boards(case).unwrap();
        let expected_rest = "".as_bytes();

        assert_eq!(3, actual_boards.len());
        assert_eq!(expected_rest, actual_rest);
    }

    #[test]
    fn test_name() {
        let case =
            " 1 16 31 46 61\n 2 17 32 47 62\n 3 18 33 48 63\n 4 19 34 49 64\n 5 20 35 50 65\n\n"
                .as_bytes();
        let (_, this_board) = board(case).unwrap();

        // short circuit cases
        assert_eq!(false, this_board.has_bingo(&[1]));
        assert_eq!(false, this_board.has_bingo(&[1, 2]));
        assert_eq!(false, this_board.has_bingo(&[1, 2, 3]));
        assert_eq!(false, this_board.has_bingo(&[1, 2, 3, 4]));

        // row
        println!("checking first row",);
        assert_eq!(true, this_board.has_bingo(&[1, 16, 31, 46, 61]));
        // not the top row
        println!("checking second row",);
        assert_eq!(true, this_board.has_bingo(&[2, 17, 32, 47, 62]));

        // 5 digits, not a bingo
        println!("checking nothing in particular",);
        assert_eq!(false, this_board.has_bingo(&[1, 2, 3, 4, 6]));

        // column
        println!("checking first column",);
        assert_eq!(true, this_board.has_bingo(&[1, 2, 3, 4, 5]));

        // more than 5 digits until we get a bingo
        assert_eq!(true, this_board.has_bingo(&[2, 17, 32, 48, 62, 47]));
        assert_eq!(true, this_board.has_bingo(&[15, 2, 17, 32, 48, 62, 47]));
        assert_eq!(
            true,
            this_board.has_bingo(&[1, 16, 31, 46, 2, 17, 32, 48, 62, 47])
        );
    }
}
