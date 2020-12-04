use std::collections::HashMap;

const INPUT: &[(&str, &str, i16)] = &[
    ("Alice", "Bob", -2),
    ("Alice", "Carol", -62),
    ("Alice", "David", 65),
    ("Alice", "Eric", 21),
    ("Alice", "Frank", -81),
    ("Alice", "George", -4),
    ("Alice", "Mallory", -80),
    ("Bob", "Alice", 93),
    ("Bob", "Carol", 19),
    ("Bob", "David", 5),
    ("Bob", "Eric", 49),
    ("Bob", "Frank", 68),
    ("Bob", "George", 23),
    ("Bob", "Mallory", 29),
    ("Carol", "Alice", -54),
    ("Carol", "Bob", -70),
    ("Carol", "David", -37),
    ("Carol", "Eric", -46),
    ("Carol", "Frank", 33),
    ("Carol", "George", -35),
    ("Carol", "Mallory", 10),
    ("David", "Alice", 43),
    ("David", "Bob", -96),
    ("David", "Carol", -53),
    ("David", "Eric", -30),
    ("David", "Frank", -12),
    ("David", "George", 75),
    ("David", "Mallory", -20),
    ("Eric", "Alice", 8),
    ("Eric", "Bob", -89),
    ("Eric", "Carol", -69),
    ("Eric", "David", -34),
    ("Eric", "Frank", 95),
    ("Eric", "George", 34),
    ("Eric", "Mallory", -99),
    ("Frank", "Alice", -97),
    ("Frank", "Bob", 6),
    ("Frank", "Carol", -9),
    ("Frank", "David", 56),
    ("Frank", "Eric", -17),
    ("Frank", "George", 18),
    ("Frank", "Mallory", -56),
    ("George", "Alice", 45),
    ("George", "Bob", 76),
    ("George", "Carol", 63),
    ("George", "David", 54),
    ("George", "Eric", 54),
    ("George", "Frank", 30),
    ("George", "Mallory", 7),
    ("Mallory", "Alice", 31),
    ("Mallory", "Bob", -32),
    ("Mallory", "Carol", 95),
    ("Mallory", "David", 91),
    ("Mallory", "Eric", -66),
    ("Mallory", "Frank", -75),
    ("Mallory", "George", -99),
];

struct Permutations<'a, T: ?Sized> {
    values: Vec<&'a T>,
    current: Option<Vec<usize>>,
}

fn permutations<T: ?Sized>(values: Vec<&T>) -> Permutations<T> {
    let current = Some(values.iter().enumerate().map(|(i, _)| i).collect());
    Permutations { values, current }
}

impl<'a, T: ?Sized> Iterator for Permutations<'a, T> {
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

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut input: HashMap<&str, HashMap<&str, i16>> = HashMap::new();
    for (from, to, diff) in INPUT {
        input.entry(from).or_default().insert(to, *diff);
    }
    let people: Vec<&str> = input.keys().copied().collect();

    let mut max_happiness = 0;
    for perm in permutations(people) {
        let mut happiness = 0;
        for (&p, &q) in perm.iter().zip(perm.iter().skip(1)) {
            happiness +=
                input.get(p).unwrap().get(q).unwrap() + input.get(q).unwrap().get(p).unwrap();
        }
        max_happiness = max_happiness.max(happiness);
    }
    println!("The maximum happiness change is {}", max_happiness);
    Ok(())
}
