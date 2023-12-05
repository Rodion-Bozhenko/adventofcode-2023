use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
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
            println!("LINE: {}", line);
            let mut iter = line.split_ascii_whitespace().skip(2);
            while let Some(x) = iter.next() {
                let count: u32 = x.parse().unwrap();
                println!("COUNT: {}", count);
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
