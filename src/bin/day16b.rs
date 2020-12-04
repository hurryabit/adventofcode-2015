use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    #[allow(clippy::type_complexity)]
    let message: &[(&str, u16, &dyn Fn(&u16, &u16) -> bool)] = &[
        ("children", 3, &u16::eq),
        ("cats", 7, &u16::gt),
        ("samoyeds", 2, &u16::eq),
        ("pomeranians", 3, &u16::lt),
        ("akitas", 0, &u16::eq),
        ("vizslas", 0, &u16::eq),
        ("goldfish", 5, &u16::lt),
        ("trees", 3, &u16::gt),
        ("cars", 2, &u16::eq),
        ("perfumes", 1, &u16::eq),
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
        let matches = message.iter().all(|(prop, expected, cmp)| props.get(prop).map_or(true, |found| cmp(found, expected)));
        if matches {
            println!("The gift is from {}", aunt);
            break;
        }
    }
    Ok(())
}
