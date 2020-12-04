use serde_json::Value;
use std::fs::File;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let reader = File::open("inputs/day12.json")?;
    let value: Value = serde_json::from_reader(reader)?;
    let mut queue = vec![value];
    let mut sum = 0;
    while let Some(value) = queue.pop() {
        match value {
            Value::Null | Value::Bool(_) | Value::String(_) => {}
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    sum += n;
                }
            }
            Value::Array(vec) => queue.extend(vec),
            Value::Object(obj) => queue.extend(obj.into_iter().map(|x| x.1)),
        }
    }
    println!("The sum of all numbers is {}", sum);
    Ok(())
}
