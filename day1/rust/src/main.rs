use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
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

    let _ = part_two();
    Ok(())
}

fn part_two() -> Result<()> {
    let file = File::open("../../input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut left: String = "".to_owned();
        let mut right: String = "".to_owned();

        for (i, c) in line.chars().enumerate() {
            if let Some(digit) = lookup_word(line.clone().get(..i).unwrap().to_owned()) {
                if left.is_empty() {
                    left = digit.to_string();
                }
            }
            if c.is_ascii_digit() && left.is_empty() {
                left = c.to_string();
            }
            if !left.is_empty() {
                sum += left.parse::<i32>().unwrap() * 10;
                break;
            }
        }
        let mut i = line.chars().count();
        for (_, c) in line.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                right = c.to_string();
            }
            if let Some(digit) = lookup_word(line.clone().get(i..).unwrap().to_owned()) {
                right = digit.to_string();
            }
            if !right.is_empty() {
                sum += right.parse::<i32>().unwrap();
                break;
            }
            i -= 1;
        }
    }
    println!("Part 2 SUM: {}", sum);

    Ok(())
}

fn lookup_word(line: String) -> Option<i32> {
    let lookup_table = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (k, _) in lookup_table.iter() {
        if line.contains(k) {
            return Some(lookup_table[k]);
        }
    }

    return None;
}
