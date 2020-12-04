use std::fs::File;
use std::io::prelude::*;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = File::open("inputs/day01.txt")?;
    let mut level = 0_i64;
    let mut position = 0;
    for (i, ch) in reader.bytes().enumerate() {
        match ch {
            Ok(b'(') => level += 1,
            Ok(b')') => level -= 1,
            _ => (),
        }
        if level < 0 {
            position = i + 1;
            break;
        }
    }
    println!("Santa enters the basement at position {}", position);
    Ok(())
}
