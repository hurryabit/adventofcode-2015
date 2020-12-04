use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = File::open("inputs/day03.txt")?;
    let mut visited = BTreeSet::new();
    let mut active_location: (i32, i32) = (0, 0);
    let mut passive_location: (i32, i32) = (0, 0);
    visited.insert(active_location);
    for ch in reader.bytes() {
        match ch? {
            b'>' => active_location.1 += 1,
            b'<' => active_location.1 -= 1,
            b'^' => active_location.0 += 1,
            b'v' => active_location.0 -= 1,
            _ => {}
        }
        visited.insert(active_location);
        std::mem::swap(&mut active_location, &mut passive_location);
    }
    println!("Santa and Robo-Stant visit {} distinct houses", visited.len());
    Ok(())
}
