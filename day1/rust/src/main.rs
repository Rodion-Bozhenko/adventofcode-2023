use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("../../input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: i64 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut left: String = "".to_owned();
        let mut right: String = "".to_owned();
        for (_, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if left.is_empty() {
                    left = c.to_string();
                    right = c.to_string();
                    sum += left.parse::<i64>().unwrap() * 10;
                } else {
                    right = c.to_string();
                }
            }
        }
        sum += right.parse::<i64>().unwrap();
    }
    println!("Part 1 SUM: {}", sum);

    Ok(())
}
