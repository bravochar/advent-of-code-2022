
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./test";


fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();
        
        println!("{}", line_str)
    }

    // Print the answer to the first part
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();

    }

    // Print the answer to the second part
}

fn main() {
    println!("Advent of Code, Day 3");

    part_1();
    part_2();
}

