
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";

fn range_from_str(s: &str) -> Result<Vec<u32>, String> {
    let range: Vec<&str> = s.split("-").collect();

    if range.len() != 2 {
        Err(format!("Error: Range '{}' did not split into 2 ranges", s))

    } else {

        let lower: u32 = range[0].parse().expect("Failed to parse range to int");
        let upper: u32 = range[1].parse().expect("Failed to parse range to int");

        Ok(vec![lower, upper])
    }
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut answer = 0;
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        // split string in 2
        let ranges: Vec<&str> = line_str.split(",").collect();
        if ranges.len() != 2 {
            println!("Error: Line '{}' did not split into 2 ranges", line_str);
            continue;
        }

        let range_1 = range_from_str(ranges[0]).expect("Failed to parse string");
        let range_2 = range_from_str(ranges[1]).expect("Failed to parse string");

        if range_1[0] >= range_2[0] && range_1[1] <= range_2[1] {
            println!("Range {:?} fits in Range {:?}", range_1, range_2);
            answer += 1;
        } else if range_2[0] >= range_1[0] && range_2[1] <= range_1[1] {
            println!("Range {:?} fits in Range {:?}", range_2, range_1);
            answer += 1;
        }
    }

    // Print the answer to the first part
    println!("First Answer: {:?}", answer);
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut answer = 0;
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        // split string in 2
        let ranges: Vec<&str> = line_str.split(",").collect();
        if ranges.len() != 2 {
            println!("Error: Line '{}' did not split into 2 ranges", line_str);
            continue;
        }

        let range_1 = range_from_str(ranges[0]).expect("Failed to parse string");
        let range_2 = range_from_str(ranges[1]).expect("Failed to parse string");

        if range_1[0] > range_2[0] && range_1[0] > range_2[1] {

        } else if range_2[0] > range_1[0] && range_2[0] > range_1[1] {

        } else {
            println!("Range {:?} overlaps Range {:?}", range_1, range_2);
            answer += 1;
        }
    }

    // Print the answer to the second part
    println!("Second Answer: {:?}", answer);
}

fn main() {
    println!("Advent of Code, Day 4");

    part_1();
    part_2();
}

