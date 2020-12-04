use std::io::prelude::*;

use anyhow::Result;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Tree,
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Self::Tree,
            _ => Self::Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::Empty => '.',
            Self::Tree => '#',
        };
        write!(f, "{}", repr)
    }
}

#[derive(Debug)]
struct Field {
    cells: Vec<Vec<Cell>>,
}

impl Field {
    pub fn from_lines(input: Vec<String>) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .iter()
            .map(|line| line.chars().map(|ch| ch.into()).collect())
            .collect();
        Self { cells }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).map(|row| row[x % row.len()])
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
        }
        Ok(())
    }
}

fn count_trees(field: &Field, x_slope: usize, y_slope: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;
    while let Some(cell) = field.at(x, y) {
        if cell == Cell::Tree {
            trees += 1;
        }

        x += x_slope;
        y += y_slope;
    }

    trees
}

#[paw::main]
fn main(args: paw::Args) -> Result<()> {
    let filename = args
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| "data/input".to_string());
    let file = BufReader::new(File::open(filename)?);

    let input: Vec<String> = file.lines().filter_map(|line| line.ok()).collect();
    let field = Field::from_lines(input);
    println!("{}", field);

    let trees = count_trees(&field, 3, 1);
    println!("{}", trees);

    let runs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = runs
        .iter()
        .map(|(x_slope, y_slope)| count_trees(&field, *x_slope, *y_slope))
        .product();

    println!("{}", product);

    Ok(())
}
