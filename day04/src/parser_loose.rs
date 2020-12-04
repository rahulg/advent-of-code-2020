use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::sequence::{delimited, preceded};
use nom::IResult;

use crate::parser::stuff;

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn byr(i: &str) -> IResult<&str, &str> {
    preceded(tag("byr:"), stuff)(i)
}

fn iyr(i: &str) -> IResult<&str, &str> {
    preceded(tag("iyr:"), stuff)(i)
}

fn eyr(i: &str) -> IResult<&str, &str> {
    preceded(tag("eyr:"), stuff)(i)
}

fn hgt(i: &str) -> IResult<&str, &str> {
    preceded(tag("hgt:"), stuff)(i)
}

fn hcl(i: &str) -> IResult<&str, &str> {
    preceded(tag("hcl:"), stuff)(i)
}

fn ecl(i: &str) -> IResult<&str, &str> {
    preceded(tag("ecl:"), stuff)(i)
}

fn pid(i: &str) -> IResult<&str, &str> {
    preceded(tag("pid:"), stuff)(i)
}

fn cid(i: &str) -> IResult<&str, &str> {
    preceded(tag("cid:"), stuff)(i)
}

pub fn record<'a>(i: &'a str) -> IResult<&'a str, bool> {
    map(
        permutation((
            ws(byr),
            ws(iyr),
            ws(eyr),
            ws(hgt),
            ws(hcl),
            ws(ecl),
            ws(pid),
            opt(ws(cid)),
        )),
        |(
            _birth_year,
            _issue_year,
            _expiration_year,
            _height,
            _hair_colour,
            _eye_colour,
            _passport_id,
            _country_id,
        )| { true },
    )(i)
}

pub fn parse(s: &str) -> Vec<bool> {
    s.split("\n\n")
        .filter_map(|r| record(r).ok().map(|(_, p)| p))
        .collect()
}
