use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

use std::str::FromStr;

fn parens(input: &str) -> IResult<&str, i64> {
    delimited(
        preceded(space0, char('(')),
        expression,
        terminated(char(')'), space0),
    )(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(delimited(space0, digit1, space0), FromStr::from_str)(input)
}

fn expression(input: &str) -> IResult<&str, i64> {
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

pub fn part1(input: String) -> i64 {
    input
        .lines()
        .filter_map(|line| expression(line).ok())
        .map(|x| x.1)
        .sum()
}

pub fn part2(input: String) {}

#[test]
fn day18_test() {
    assert_eq!(12, expression("5 + 4 + 3").unwrap().1);
    assert_eq!(Ok(("", 60)), expression("5 * 4 * 3"));
    assert_eq!(Ok(("", 26)), expression("2 * 3 + (4 * 5)"));
}
