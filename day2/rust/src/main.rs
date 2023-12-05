use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let _ = part_two();
    part_one()
}

fn part_one() -> Result<()> {
    let input_file = File::open("../../input.txt")?;
    let reader = BufReader::new(input_file);

    let colors: HashMap<_, _> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    let result = reader
        .lines()
        .enumerate()
        .filter_map(|(n, line)| {
            let line = line.unwrap();
            let mut iter = line.split_ascii_whitespace().skip(2);
            while let Some(x) = iter.next() {
                let count: u32 = x.parse().unwrap();
                let color = iter.next().unwrap().trim_end_matches([',', ';']);
                if count > colors[color] {
                    return None;
                }
            }
            Some(n + 1)
        })
        .sum::<usize>();

    println!("PART 1 RESULT: {}", result);

    Ok(())
}

fn part_two() -> Result<()> {
    let input_file = File::open("../../input.txt")?;
    let reader = BufReader::new(input_file);

    let result: u32 = reader
        .lines()
        .map(|line| {
            let mut colors: HashMap<String, u32> = [
                (String::from("red"), 0),
                (String::from("green"), 0),
                (String::from("blue"), 0),
            ]
            .into_iter()
            .collect();
            let line: &str = &line.unwrap();
            let mut iter = line.split_ascii_whitespace().skip(2);
            while let Some(x) = iter.next() {
                let count: u32 = x.parse().unwrap();
                let color = iter.next().unwrap().trim_end_matches([',', ';']);
                let max = colors.get_mut(color).unwrap();
                if count > *max {
                    *max = count;
                }
            }
            colors.into_iter().map(|(_, count)| count).product::<u32>()
        })
        .sum();

    println!("PART 2 RESULT: {}", result);

    Ok(())
}
