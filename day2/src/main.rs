use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), String> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), String> {
    let contents = fs::read_to_string("input").expect("Reading file process failed");
    let mut twice = 0;
    let mut three_times = 0;
    for line in contents.split_whitespace() {
        let mut times_recorder = HashMap::new();
        for ch in line.chars() {
            times_recorder
                .entry(ch)
                .and_modify(|value| {
                    *value += 1;
                })
                .or_insert(1);
        }
        if times_recorder.values().any(|e| *e == 2) {
            twice += 1;
        }
        if times_recorder.values().any(|e| *e == 3) {
            three_times += 1;
        }
    }

    // calculate the final checksum
    println!("Checksum is {}", twice * three_times);
    Ok(())
}

fn part2() -> Result<(), String> {
    let strings = read_contents_from_file("input");
    for i in 0..strings.len() {
        for j in i + 1..strings.len() {
            if has_one_char_different(&strings[i], &strings[j]) {
                println!(
                    "Pair with one character difference: {} and {}",
                    strings[i], strings[j]
                );
            }
        }
    }
    Ok(())
}

fn read_contents_from_file(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}

fn has_one_char_different(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut differences = 0;
    let chars1 = s1.chars();
    let chars2 = s2.chars();

    for (c1, c2) in chars1.zip(chars2) {
        if c1 != c2 {
            differences += 1;
            if differences > 1 {
                return false;
            }
        }
    }

    differences == 1
}
