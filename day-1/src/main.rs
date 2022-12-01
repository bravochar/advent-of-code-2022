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
    let mut num_elves = 0;
    let mut max_calories = 0;
    let mut elf_calories: Vec<i32> = vec![];

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();

        // On a blank line, check the sum against the previous max
        if line_str.is_empty() {
            num_elves += 1;

            // If the new max is hire, save it and continue through the file
            if sum > max_calories {
                max_calories = sum;
            }
            elf_calories.push(sum);
            sum = 0;

        } else {
            // Sum the numbers on each line
            sum += line_str.parse::<i32>().unwrap()
        }
    }

    // Print the answer to the first part
    println!("Of {:?} elves, the Elf with the most has: {:?} Calories", num_elves, max_calories);

    // Sort the vector? then print the top 3
    elf_calories.sort();
    let mut top_three_sum = 0;
    for cal in &elf_calories[num_elves - 3..] {
        top_three_sum += cal;
    }
    println!("Of {:?} elves, the top 3 elves have: {:?} Calories", num_elves, top_three_sum);
}
