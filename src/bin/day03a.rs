use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = File::open("inputs/day03.txt")?;
    let mut visited = BTreeSet::new();
    let mut location: (i32, i32) = (0, 0);
    visited.insert(location);
    for ch in reader.bytes() {
        match ch? {
            b'>' => location.1 += 1,
            b'<' => location.1 -= 1,
            b'^' => location.0 += 1,
            b'v' => location.0 -= 1,
            _ => {}
        }
        visited.insert(location);
    }
    println!("Santa visits {} distinct houses", visited.len());
    Ok(())
}
