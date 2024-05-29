const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::cmp::Ordering;
use std::{fmt, vec};
use std::fs::{read, File};
use std::io::{prelude::*, BufReader};

#[derive(Clone)]
struct Blueprint {
    id: i32,
    ore_robot: [i32; 3],
    clay_robot: [i32; 3],
    obsidian_robot: [i32; 3],
    geode_robot: [i32; 3],
    best_score: i32
}

impl Blueprint {

    fn from_line(line: &str) -> Blueprint {
        let (_, line) = line.split_once(" ").unwrap();
        let (id, line) = line.split_once(":").unwrap();
        let id = id.parse().unwrap();
        let (_, line) = line.split_once("costs ").unwrap();

        Blueprint {
            id,

            best_score: 0
        }
    }
}

fn read_file(filename: &str) -> Vec<Blueprint> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        rval.push(Blueprint::from_line(line));
    }
    rval
}

fn part_1(mut blueprints: Vec<Blueprint>) -> i32 {

    0
}

fn part_2(mut blueprints: Vec<Blueprint>) -> i32 {

    0
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let blueprints = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(blueprints.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 33);
    }

    let now = Instant::now();
    let answer = part_2(blueprints.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        //assert_eq!(answer, 58);
    }
}
