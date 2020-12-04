use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let message = &[
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    let reader = BufReader::new(File::open("inputs/day16.txt")?);
    for line in reader.lines() {
        let line = line?;
        let colon = line.find(':').unwrap();
        let aunt = &line[..colon];
        let props: HashMap<&str, u16> = line[colon+2..].split(", ").map(|pair| {
            let colon = pair.find(':').unwrap();
            let prop = &pair[..colon];
            let value: u16 = pair[colon+2..].parse().unwrap();
            (prop, value)
        }).collect();
        let matches = message.iter().all(|(prop, expected)| props.get(prop).map_or(true, |found| expected == found));
        if matches {
            println!("The gift is from {}", aunt);
            break;
        }
    }
    Ok(())
}
