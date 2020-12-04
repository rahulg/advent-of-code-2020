use std::io::prelude::*;

use anyhow::Result;
use std::fs::File;

mod parser;
mod parser_loose;

#[derive(Debug, PartialEq)]
pub struct Passport {
    pub birth_year: u16,
    pub issue_year: u16,
    pub expiration_year: u16,
    pub height: (u16, String),
    pub hair_colour: (u8, u8, u8),
    pub eye_colour: String,
    pub passport_id: usize,
    pub country_id: Option<String>,
}

#[paw::main]
fn main(args: paw::Args) -> Result<()> {
    let filename = args
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| "data/input".to_string());
    let mut input = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut input)?;

    let passports_loose = parser_loose::parse(&input);
    println!("{}", passports_loose.len());

    let passports = parser::parse(&input);
    println!("{}", passports.len());

    Ok(())
}
