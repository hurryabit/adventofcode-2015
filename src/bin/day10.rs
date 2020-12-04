use std::iter::{successors, Peekable};

type Error = Box<dyn std::error::Error>;

struct Runs<I: Iterator> {
    iter: Peekable<I>,
}

fn runs<I: Iterator>(iter: I) -> Runs<I> {
    Runs {
        iter: iter.peekable(),
    }
}

impl<I> Iterator for Runs<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let mut count = 1;
            while self.iter.peek().map_or(false, |peeked| peeked == &item) {
                count += 1;
                self.iter.next();
            }
            Some((count, item))
        } else {
            None
        }
    }
}

fn step(input: &str) -> String {
    let mut output = String::new();
    for (n, ch) in runs(input.chars()) {
        output.push_str(&n.to_string());
        output.push(ch);
    }
    output
}

fn main() -> Result<(), Error> {
    let input = "1113122113".to_string();
    let result = successors(Some(input), |s| Some(step(s))).nth(50).unwrap();
    println!("The length of the result is {}", result.len());
    Ok(())
}
