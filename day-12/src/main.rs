
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./test";

#[derive(Debug)]
struct Hill {
    h_char: char,
    height: u32,
}

impl Hill {
    fn from_height(h_char: char) -> Self {
        let height: u32 = match h_char {
            'S' => 0,
            'E' => 25,
            'a'..='z' => h_char as u32 - 'a' as u32,
            _ => panic!("Invalid height charcter {:?}", h_char),

        };

        Self {
            h_char,
            height,
        }
    }
}

// read in the file and store in a 2-D vector
fn read_hill_grid() -> Vec::<Vec::<Hill>> {
    let mut rows = Vec::<Vec::<Hill>>::new();

    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let line_str = line_str.trim();
        println!("{}", line_str);

        let mut row = Vec::<Hill>::new();
        for c in line_str.chars() {
            row.push(Hill::from_height(c));
        }
        rows.push(row);
    }

    rows
}


// TODO: function to check for valid move
// TODO: make function an `impl` for Hill

fn part_1() {

    let hills = read_hill_grid();

    // Our goal is find the minimum path from the start to the finish
    //  * Can move to any position that is no more than 1 higher than our
    //    current position (can move down any number)
    //
    // We can mark positions on the map as visited when any path touches them
    // because if we've already been there, then we have a shorter path to get
    // to that point and the move is not part of the optimal path
    //

}


fn main() {
    println!("Advent of Code, Day 12");

    part_1();
}


