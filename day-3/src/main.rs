
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";

fn letter_to_priority(c: char) -> u32 {
    let mut pri = c as u32;

    pri -= 'A' as u32;
    pri += 1;

    if pri > 26 {
        // a-z starts at 1, so we subtract the difference of 'a' and 'A'
        pri -= 0x20;
    } else {
        // A-Z starts at 27
        pri += 26;
    }

    pri
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut all_dups = Vec::new();
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        // split string in 2
        let comp_len = line_str.len() / 2;
        let comp_1 = &line_str[..comp_len];
        let comp_2 = &line_str[comp_len..];

        // find common characters
        let mut duplicates = Vec::new();
        for c in comp_1.chars() {
            if comp_2.contains(c) && !duplicates.contains(&c) {
                duplicates.push(c);
                all_dups.push(c)
            }
        }
    }


    // create sum
    let mut sum = 0;
    for d in all_dups.iter() {
        let pri = letter_to_priority(*d);
        sum += pri;
    }

    // Print the answer to the first part
    println!("Answer: {:?}", sum);
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        println!("{}", line_str)
    }

    // Print the answer to the second part
}

fn main() {
    println!("Advent of Code, Day 3");

    part_1();
    //part_2();
}

