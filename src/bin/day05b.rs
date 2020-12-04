use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day05.txt")?);
    let mut nice_count = 0_u32;
    for line in reader.lines() {
        let line = line?;
        let line = line.as_bytes();
        let mut seen_pairs: BTreeMap<(u8, u8), usize> = BTreeMap::new();
        seen_pairs.insert((line[0], line[1]), 1);
        let mut double_pair = false;
        let mut sep_1_rep = false;
        for i in 2..line.len() {
            let pair = (line[i-1], line[i]);
            if let Some(j) = seen_pairs.get(&pair) {
                if i >= j + 2 {
                    double_pair = true;
                }
            } else {
                seen_pairs.insert(pair, i);
            }

            if line[i-2] == line[i] {
                sep_1_rep = true;
            }
        }
        if double_pair && sep_1_rep {
            nice_count += 1;
        }
    }
    println!("There are {} nice strings", nice_count);
    Ok(())
}
