type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let secret = "yzbqklnj";
    let mut counter = 1;
    loop {
        let digest = format!("{:x}", md5::compute(format!("{}{}", secret, counter)));
        if digest.starts_with("000000") {
            break;
        }
        counter += 1;
    }
    println!("The number is {}", counter);
    Ok(())
}
