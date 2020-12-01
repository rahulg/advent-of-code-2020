use std::io::prelude::*;

use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;

#[paw::main]
fn main(args: paw::Args) -> Result<()> {
    let filename = args
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| "data/input".to_string());
    let file = BufReader::new(File::open(filename)?);

    let entries: Vec<i64> = file
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse().ok())
        .collect();

    let muls_1: Vec<_> = entries
        .iter()
        .tuple_combinations()
        .filter_map(|(&a, &b)| if a + b == 2020 { Some(a * b) } else { None })
        .collect();

    println!("part one");
    for mul in muls_1 {
        println!("{}", mul);
    }
    println!("======");

    let muls_2: Vec<_> = entries
        .iter()
        .tuple_combinations()
        .filter_map(|(&a, &b, &c)| {
            if a + b + c == 2020 {
                Some(a * b * c)
            } else {
                None
            }
        })
        .collect();

    println!("part two");
    for mul in muls_2 {
        println!("{}", mul);
    }
    println!("======");

    Ok(())
}
