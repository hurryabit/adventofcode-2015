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

struct ReindeerStatus {
    dist: i16,
    points: i16,
    on: bool,
    remaining: i16,
    speed: i16,
    on_duration: i16,
    off_duration: i16,
}

fn main() -> Result<(), Error> {
    let mut reindeers: Vec<ReindeerStatus> = REINDEERS
        .iter()
        .map(|(_, speed, on, off)| ReindeerStatus {
            dist: 0,
            points: 0,
            on: true,
            remaining: *on,
            speed: *speed,
            on_duration: *on,
            off_duration: *off,
        })
        .collect();

    for _ in 0..DURATION {
        for r in &mut reindeers {
            if r.on {
                r.dist += r.speed;
            }
            r.remaining -= 1;
            if r.remaining == 0 {
                r.on = !r.on;
                r.remaining = if r.on { r.on_duration } else { r.off_duration };
            }
        }
        let leader_dist = reindeers.iter().map(|r| r.dist).max().unwrap();
        for r in &mut reindeers {
            if r.dist == leader_dist {
                r.points += 1;
            }
        }
    }
    let max_points = reindeers.iter().map(|r| r.points).max().unwrap();
    println!("The maximum number of point is {}", max_points);
    Ok(())
}
