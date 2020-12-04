use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day05.txt")?);
    let vowels: BTreeSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().copied().collect();
    let forbidden_pairs: BTreeSet<(char, char)> = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')]
        .iter()
        .copied()
        .collect();
    let mut nice_count = 0_u32;
    for line in reader.lines() {
        let line = line?;
        let mut num_vowels = 0_u32;
        let mut has_double = false;
        let mut has_forbidden = false;
        let mut last_char = '\0';
        for ch in line.chars() {
            if vowels.contains(&ch) {
                num_vowels += 1;
            }
            if last_char == ch {
                has_double = true;
            }
            if forbidden_pairs.contains(&(last_char, ch)) {
                has_forbidden = true;
            }
            last_char = ch;
        }
        if num_vowels >= 3 && has_double && !has_forbidden {
            nice_count += 1;
        }
    }
    println!("There are {} nice strings", nice_count);
    Ok(())
}
