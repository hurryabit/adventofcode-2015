use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn with_subgrid(
    grid: &mut Vec<Vec<bool>>,
    row1: usize,
    col1: usize,
    row2: usize,
    col2: usize,
    f: impl Fn(bool) -> bool,
) {
    #[allow(clippy::needless_range_loop)]
    for row in row1..=row2 {
        for col in col1..=col2 {
            let cell = &mut grid[row][col];
            *cell = f(*cell);
        }
    }
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day06.txt")?);
    let regex = Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)")?;
    let mut grid = Vec::new();
    grid.resize_with(1000, || {
        let mut line = Vec::new();
        line.resize(1000, false);
        line
    });

    for line in reader.lines() {
        let line = line?;
        let caps = regex.captures(&line).unwrap();
        let instr = caps.get(1).unwrap().as_str();
        let row1 = caps.get(2).unwrap().as_str().parse().unwrap();
        let col1 = caps.get(3).unwrap().as_str().parse().unwrap();
        let row2 = caps.get(4).unwrap().as_str().parse().unwrap();
        let col2 = caps.get(5).unwrap().as_str().parse().unwrap();
        match instr {
            "toggle" => with_subgrid(&mut grid, row1, col1, row2, col2, |b| !b),
            "turn on" => with_subgrid(&mut grid, row1, col1, row2, col2, |_| true),
            "turn off" => with_subgrid(&mut grid, row1, col1, row2, col2, |_| false),
            _ => {}
        };
    }
    let num_lights_on = grid
        .iter()
        .flat_map(|line| line.iter())
        .filter(|x| **x)
        .count();

    println!("{} lights are turned on", num_lights_on);
    Ok(())
}
