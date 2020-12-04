use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day08.txt")?);
    let mut overhead: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        overhead += 2;
        let mut escaped = false;
        for ch in line.chars() {
            if escaped {
                overhead += 1;
                if ch == 'x' {
                    overhead += 2;
                }
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            }
        }
    }
    println!("The overhead is {} characters", overhead);
    Ok(())
}
