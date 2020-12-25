use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use crate::Part;
use std::str::FromStr;

fn number(input: &str) -> IResult<&str, i64> {
    map_res(delimited(space0, digit1, space0), FromStr::from_str)(input)
}

pub fn solutions(input: String, part: Part) -> i64 {
    let expression = match part {
        Part::Part1 => part1::expression,
        Part::Part2 => part2::expression,
    };

    input
        .lines()
        .filter_map(|line| expression(line).ok())
        .map(|x| x.1)
        .sum()
}

mod part1 {
    use super::*;

    fn parens(input: &str) -> IResult<&str, i64> {
        delimited(
            preceded(space0, char('(')),
            expression,
            terminated(char(')'), space0),
        )(input)
    }

    pub(super) fn expression(input: &str) -> IResult<&str, i64> {
        let (i, init) = alt((number, parens))(input)?;

        fold_many0(
            pair(alt((char('+'), char('*'))), alt((number, parens))),
            init,
            |acc, (op, val): (char, i64)| match op {
                '+' => acc + val,
                '*' => acc * val,
                _ => panic!("Unrecognized operand: {:?}", op),
            },
        )(i)
    }

    #[test]
    fn day18_test1() {
        assert_eq!(12, expression("5 + 4 + 3").unwrap().1);
        assert_eq!(Ok(("", 60)), expression("5 * 4 * 3"));
        assert_eq!(Ok(("", 26)), expression("2 * 3 + (4 * 5)"));
    }
}

mod part2 {
    use super::*;

    fn parens(input: &str) -> IResult<&str, i64> {
        delimited(
            preceded(space0, char('(')),
            expression,
            terminated(char(')'), space0),
        )(input)
    }

    fn term(input: &str) -> IResult<&str, i64> {
        let (i, init) = alt((number, parens))(input)?;

        fold_many0(
            pair(char('+'), alt((number, parens))),
            init,
            |acc, (_op, val): (char, i64)| acc + val,
        )(i)
    }

    pub(super) fn expression(input: &str) -> IResult<&str, i64> {
        let (i, init) = term(input)?;

        fold_many0(
            pair(char('*'), term),
            init,
            |acc, (_op, val): (char, i64)| acc * val,
        )(i)
    }

    #[test]
    fn day18_test2() {
        dbg!(expression("1 + 2 * 3 + 4 * 5 + 6"));
    }
}
