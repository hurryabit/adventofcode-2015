const REINDEERS: &[(&str, i16, i16, i16)] = &[
    ("Dancer", 27, 5, 132),
    ("Cupid", 22, 2, 41),
    ("Rudolph", 11, 5, 48),
    ("Donner", 28, 5, 134),
    ("Dasher", 4, 16, 55),
    ("Blitzen", 14, 3, 38),
    ("Prancer", 3, 21, 40),
    ("Comet", 18, 6, 103),
    ("Vixen", 18, 5, 84),
];

const DURATION: i16 = 2503;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let mut max_dist = 0;
    for (_, speed, on, off) in REINDEERS {
        let num_full_cycles = DURATION / (on + off);
        let partial_cycle_on = (DURATION % (on + off)).min(*on);
        let dist = (num_full_cycles * on + partial_cycle_on) * speed;
        max_dist = max_dist.max(dist);
    }
    println!("The maximum distance is {}", max_dist);
    Ok(())
}
