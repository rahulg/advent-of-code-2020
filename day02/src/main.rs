use std::io::prelude::*;

use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufReader;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("failed to parse a record")]
struct ParseFailure {}

fn split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
    let ret: Vec<_> = s.splitn(2, delim).take(2).collect();
    match &ret[..] {
        [a, b] => Some((a, b)),
        _ => None,
    }
}

#[derive(Debug)]
struct Policy {
    a: usize,
    b: usize,
    token: char,
}

impl Policy {
    pub fn try_parse(repr: &str) -> Result<Self> {
        let (counts, token_s) = split_once(repr, " ").ok_or(ParseFailure {})?;
        let (a_s, b_s) = split_once(counts, "-").ok_or(ParseFailure {})?;

        Ok(Self {
            a: a_s.parse()?,
            b: b_s.parse()?,
            token: token_s.chars().next().ok_or(ParseFailure {})?,
        })
    }

    pub fn validate(&self, pwd: &str) -> bool {
        let n = pwd.matches(self.token).count();
        self.a <= n && n <= self.b
    }

    pub fn validate2(&self, pwd: &str) -> bool {
        match self.validate2_inner(pwd) {
            Ok(b) => b,
            Err(_) => false,
        }
    }

    pub fn validate2_inner(&self, pwd: &str) -> Result<bool> {
        let char_a = pwd.chars().nth(self.a - 1).ok_or_else(|| anyhow!(""))?;
        let char_b = pwd.chars().nth(self.b - 1).ok_or_else(|| anyhow!(""))?;

        Ok((char_a == self.token) ^ (char_b == self.token))
    }
}

#[paw::main]
fn main(args: paw::Args) -> Result<()> {
    let filename = args
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| "data/input".to_string());
    let file = BufReader::new(File::open(filename)?);

    let db: Vec<_> = file
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            split_once(&line, ": ").map(|(ref a, ref b)| (a.to_string(), b.to_string()))
        })
        .filter_map(|(policy_s, pwd)| {
            Policy::try_parse(&policy_s)
                .ok()
                .map(|policy| (policy, pwd))
        })
        .collect();

    let old = db
        .iter()
        .filter(|(policy, pwd)| policy.validate(pwd))
        .count();

    println!("{}", old);

    let new = db
        .iter()
        .filter(|(policy, pwd)| policy.validate2(pwd))
        .count();

    println!("{}", new);

    Ok(())
}
