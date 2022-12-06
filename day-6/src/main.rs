use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

const FILENAME: &str = "./input";

fn start_of_packet(x: &str) -> Option<usize> {
    let mut seq = VecDeque::<char>::new();

    for (i, c) in x.chars().enumerate() {
        if !seq.contains(&c) && seq.len() == 3 {
            // if this is the fourth unique character, then this is the start of packet
            return Some(i + 1);

        } else if seq.contains(&c) {
            // remove all characters at the beginning of the sequence until we
            // remove the repeated character
            while seq.contains(&c) {
                seq.pop_front();
            }
        }

        // insert at zero for a FIFO queue
        seq.push_back(c);
    }

    None
}

fn start_of_message(x: &str) -> Option<usize> {
    let mut seq = VecDeque::<char>::new();

    for (i, c) in x.chars().enumerate() {
        if !seq.contains(&c) && seq.len() == 13 {
            // if this is the fourteenth unique character, then this is the start of message
            return Some(i + 1);

        } else if seq.contains(&c) {
            // remove all characters at the beginning of the sequence until we
            // remove the repeated character
            while seq.contains(&c) {
                seq.pop_front();
            }
        }

        // insert at zero for a FIFO queue
        seq.push_back(c);
    }

    None
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut answer = 0;

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        let start_char = start_of_packet(&line_str);
        if let Some(i) = start_char {
            answer = i;
        }
    }

    // Print the answer to the first part
    println!("First Answer: {:?}", answer);
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut answer = 0;

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        let start_char = start_of_message(&line_str);
        if let Some(i) = start_char {
            answer = i;
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

