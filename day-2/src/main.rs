use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

fn str_to_throw(s: &str) -> Throw {
    match s {
        "A" => Throw::Rock,
        "X" => Throw::Rock,
        "B" => Throw::Paper,
        "Y" => Throw::Paper,
        "C" => Throw::Scissors,
        "Z" => Throw::Scissors,
        &_ => Throw::Rock,
    }
}

fn throw_to_points(our_throw: &Throw) -> i32 {
    match our_throw {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    }
}

fn outcome_to_points(our_throw: &Throw, opp_throw: &Throw) -> i32 {
    match our_throw {
        Throw::Rock => {
            match opp_throw {
                Throw::Rock => 3,
                Throw::Paper => 0,
                Throw::Scissors => 6,
            }
        },
        Throw::Paper => {
            match opp_throw {
                Throw::Rock => 6,
                Throw::Paper => 3,
                Throw::Scissors => 0,
            }
        },
        Throw::Scissors => {
            match opp_throw {
                Throw::Rock => 0,
                Throw::Paper => 6,
                Throw::Scissors => 3,
            }
        },
    }
}

fn part_1() {
    // Open the file
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    let mut cum_score = 0;

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();
        let parts = line_str.split(" ").collect::<Vec<&str>>();
        if parts.len() < 2 {
            println!("Error parsing line {:?}", line_str);
            continue;
        }

        let opp_throw = str_to_throw(parts[0]);
        let our_throw = str_to_throw(parts[1]);

        let mut score = throw_to_points(&our_throw);
        score += outcome_to_points(&our_throw, &opp_throw);

        cum_score += score;
    }

    // Print the answer to the first part
    println!("Final score: {}", cum_score);
}

fn throw_from_result(opp_throw: &Throw, result: &str) -> Throw {
    match result {
        "X" => { // need to lose
            match opp_throw {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Paper,
            }
        },
        "Y" => { // need to draw
            match opp_throw {
                Throw::Rock => Throw::Rock,
                Throw::Paper => Throw::Paper,
                Throw::Scissors => Throw::Scissors,
            }
        },
        "Z" => { // need to win
            match opp_throw {
                Throw::Rock => Throw::Paper,
                Throw::Paper => Throw::Scissors,
                Throw::Scissors => Throw::Rock,
            }
        },
        &_ => Throw::Rock,
    }
}

fn part_2() {
    // Open the file
    let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    let mut cum_score = 0;

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.unwrap();
        let parts = line_str.split(" ").collect::<Vec<&str>>();
        if parts.len() < 2 {
            println!("Error parsing line {:?}", line_str);
            continue;
        }

        let opp_throw = str_to_throw(parts[0]);
        let our_throw = throw_from_result(&opp_throw, parts[1]);

        let mut score = throw_to_points(&our_throw);
        score += outcome_to_points(&our_throw, &opp_throw);

        cum_score += score;
    }

    // Print the answer to the first part
    println!("Final score: {}", cum_score);
}

fn main() {
    println!("Advent of Code, Day 2");

    part_1();
    part_2();
}
