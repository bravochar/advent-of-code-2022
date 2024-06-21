const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn snafu_to_dec(snafu: &str) -> i64 {
    let mut rval = 0;

    for c in snafu.chars() {
        rval *= 5;
        match c {
            '=' => rval -= 2,
            '-' => rval -= 1,
            '1' => rval += 1,
            '2' => rval += 2,
            _ => ()
        }
    }

    rval
}

fn dec_to_snafu(dec: i64) -> String {
    let mut base_5 = Vec::new();
    
    // first - make it base 5
    let mut q = dec;
    while q > 0 {
        let r = q % 5;
        q /= 5;
        base_5.push(r);
    }

    let mut rval = String::new();
    let mut carry = 0;
    for mut d in base_5 {
        d += carry;

        match d {
            5 => {
                carry = 1;
                rval.push('0');
            },
            4 => {
                carry = 1;
                rval.push('-');
            },
            3 => {
                carry = 1;
                rval.push('=');
            },
            2 => {
                carry = 0;
                rval.push('2');
            },
            1 => {
                carry = 0;
                rval.push('1');
            },
            0 => {
                carry = 0;
                rval.push('0');
            },
            _ => panic!("Math is broken?")
        };
    }
    if carry == 1 {
        rval.push('1');
    }

    rval.chars().rev().collect()
}

fn read_file(filename: &str) -> Vec<i64> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let dec = snafu_to_dec(line.trim());
        rval.push(dec);

        //println!("{line} -> {dec} -> {}", dec_to_snafu(dec));
    }

    rval
}


fn part_1(input: Vec<i64>) -> String {
    
    let sum = input.iter().sum();

    dec_to_snafu(sum)
}

fn part_2(mut _input: Vec<i64>) -> i64 {
    0
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let input = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(input.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, "2=-1=0");
    } else {
        //assert_eq!(answer, 255);
    }

    let now = Instant::now();
    let answer = part_2(input.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        //assert_eq!(answer, 54);
    } else {
        //assert_eq!(answer, 809);
    }
}