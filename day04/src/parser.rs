use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::character::complete::{alpha1, digit1, multispace0};
use nom::character::{is_newline, is_space};
use nom::combinator::{map, map_res, opt, verify};
use nom::error::ParseError;
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

use super::Passport;

pub(crate) fn stuff(i: &str) -> IResult<&str, &str> {
    take_while1(|c| !is_space(c as u8) && !is_newline(c as u8))(i)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn u16_parser(i: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse())(i)
}

fn num_4d(i: &str) -> IResult<&str, u16> {
    map_res(take_while_m_n(4, 4, |c: char| c.is_digit(10)), |s: &str| {
        s.parse()
    })(i)
}

fn byr(i: &str) -> IResult<&str, u16> {
    preceded(tag("byr:"), verify(num_4d, |b| 1920 <= *b && *b <= 2002))(i)
}

fn iyr(i: &str) -> IResult<&str, u16> {
    preceded(tag("iyr:"), verify(num_4d, |b| 2010 <= *b && *b <= 2020))(i)
}

fn eyr(i: &str) -> IResult<&str, u16> {
    preceded(tag("eyr:"), verify(num_4d, |b| 2020 <= *b && *b <= 2030))(i)
}

fn hgt(i: &str) -> IResult<&str, (u16, &str)> {
    preceded(
        tag("hgt:"),
        verify(pair(u16_parser, alpha1), |(ht, unit)| match *unit {
            "cm" => 150 <= *ht && *ht <= 193,
            "in" => 59 <= *ht && *ht <= 76,
            _ => false,
        }),
    )(i)
}

fn hex2_parser(i: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |c: char| c.is_digit(16)), |s| {
        u8::from_str_radix(s, 16)
    })(i)
}

fn hex_colour(i: &str) -> IResult<&str, (u8, u8, u8)> {
    preceded(tag("#"), tuple((hex2_parser, hex2_parser, hex2_parser)))(i)
}

fn hcl(i: &str) -> IResult<&str, (u8, u8, u8)> {
    preceded(tag("hcl:"), hex_colour)(i)
}

fn ecl(i: &str) -> IResult<&str, &str> {
    preceded(
        tag("ecl:"),
        alt((
            tag("amb"),
            tag("blu"),
            tag("brn"),
            tag("gry"),
            tag("grn"),
            tag("hzl"),
            tag("oth"),
        )),
    )(i)
}

fn pid(i: &str) -> IResult<&str, usize> {
    preceded(
        tag("pid:"),
        map_res(take_while_m_n(9, 9, |c: char| c.is_digit(10)), |s: &str| {
            s.parse()
        }),
    )(i)
}

fn cid(i: &str) -> IResult<&str, &str> {
    preceded(tag("cid:"), stuff)(i)
}

pub fn record<'a>(i: &'a str) -> IResult<&'a str, Passport> {
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
            birth_year,
            issue_year,
            expiration_year,
            (h_num, h_unit),
            hair_colour,
            eye_colour,
            passport_id,
            country_id,
        )| {
            Passport {
                birth_year,
                issue_year,
                expiration_year,
                height: (h_num, h_unit.to_string()),
                hair_colour,
                eye_colour: eye_colour.to_string(),
                passport_id,
                country_id: country_id.map(|s| s.to_string()),
            }
        },
    )(i)
}

pub fn parse(s: &str) -> Vec<Passport> {
    s.split("\n\n")
        .filter_map(|r| record(r).ok().map(|(_, p)| p))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_passports() {
        let samples = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ];

        for s in samples {
            assert!(record(&s).is_err());
        }
    }

    #[test]
    fn valid_passports() {
        let samples = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];

        for s in samples {
            record(&s).unwrap();
        }
    }
}
