use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day02.txt")?);
    let mut need: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut dims: Vec<u32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        dims.sort_unstable();
        need += 2 * dims[0] + 2 * dims[1] + dims[0] * dims[1] * dims[2];
    }
    println!("The elves need {} ft of ribbon", need);
    Ok(())
}
