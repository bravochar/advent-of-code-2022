use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn main() {
    println!("Advent of Code, Day 1");

    // Get input filename from commandline args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {:?}", filename);

    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut elf_calories: Vec<i32> = vec![];
    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();

        // On a blank line, check the sum against the previous max
        if line_str.is_empty() {
            elf_calories.push(sum);
            sum = 0;

        } else {
            // Sum the numbers on each line
            sum += line_str.parse::<i32>().unwrap()
        }
    }

    // Sort the vector, largest first
    elf_calories.sort_by(|a, b| b.cmp(a));

    // Print the answer to the first part
    println!("The Elf with the most has: {:?} Calories",
        elf_calories.first().unwrap());

    println!("The top 3 elves have: {} Calories",
        elf_calories.iter().take(3).sum::<i32>());
}
