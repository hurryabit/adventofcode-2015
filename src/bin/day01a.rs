use std::fs::File;
use std::io::prelude::*;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = File::open("inputs/day01.txt")?;
    let mut level = 0_i64;
    for ch in reader.bytes() {
        match ch {
            Ok(b'(') => level += 1,
            Ok(b')') => level -= 1,
            _ => (),
        }
    }
    println!("Santa ends up on level {}", level);
    Ok(())
}
