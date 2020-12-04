use std::collections::BTreeMap;

type Error = Box<dyn std::error::Error>;

const CONTAINERS: &[u16] = &[
    11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3,
];

const TARGET: u16 = 150;

fn main() -> Result<(), Error> {
    let mut counts = BTreeMap::new();
    for set in 0_u32..0xfffff {
        let sum: u16 = CONTAINERS
            .iter()
            .enumerate()
            .map(|(i, c)| if set & (1 << i) != 0 { *c } else { 0 })
            .sum();
        if sum == TARGET {
            *counts.entry(set.count_ones()).or_insert(0) += 1;
        }
    }
    println!("There are {} combinations", counts.values().next().unwrap());
    Ok(())
}
