use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Error = Box<dyn std::error::Error>;

struct Permutations<'a, T> {
    values: Vec<&'a T>,
    current: Option<Vec<usize>>,
}

fn permutations<T>(values: Vec<&T>) -> Permutations<T> {
    let current = Some(values.iter().enumerate().map(|(i, _)| i).collect());
    Permutations { values, current }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.as_mut() {
            let values = &self.values;
            let result = current.iter().map(|j| values[*j]).collect();
            let mut i = current.len();
            while i > 0 {
                i -= 1;
                if i == 0 {
                    self.current = None;
                    break; // We need to break to make the borrow checker happy.
                } else if current[i - 1] < current[i] {
                    let mut j = i + 1;
                    while j < current.len() {
                        if current[i - 1] > current[j] {
                            break;
                        }
                        j += 1;
                    }
                    current.swap(i - 1, j - 1);
                    current[i..].reverse();
                    break;
                }
            }
            Some(result)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Error> {
    let reader = BufReader::new(File::open("inputs/day09.txt")?);
    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let from = words[0].to_string();
        let to = words[2].to_string();
        let dist: u32 = words[4].parse()?;
        distances.entry(from.clone()).or_default().insert(to.clone(), dist);
        distances.entry(to).or_default().insert(from, dist);
    }
    let mut max_dist = 0;
    for route in permutations(distances.keys().collect::<Vec<_>>()) {
        let mut dist = 0;
        for (from, to) in route.iter().zip(route.iter().skip(1)) {
            dist += distances.get(*from).unwrap().get(*to).unwrap();
        }
        max_dist = max_dist.max(dist);
    }

    println!("The longest route has length {}", max_dist);

    Ok(())
}
