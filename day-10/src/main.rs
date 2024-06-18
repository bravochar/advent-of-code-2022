use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::hash::Hash;

const FILENAME: &str = "./input";

#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
enum Instr {
    Addx(i32),
    Noop,
}

fn instr_from_line(line_str: String) -> Instr {
    let splits: Vec<&str> = line_str.split(' ').collect();

    if splits.len() == 2 && splits[0] == "addx" {
        let mag: i32 = splits[1].parse().expect("Couldn't parse move magnitude");

        Instr::Addx(mag)
    } else if splits.len() == 1 && splits[0] == "noop" {
        Instr::Noop
    } else {
        panic!("Illegal instruction?");
    }
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut reg_status = Vec::<i32>::new();

    // append two 1's to remove 0-indexing headache
    reg_status.push(1);
    reg_status.push(1);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let instr = instr_from_line(line_str);
        
        let reg_val = *reg_status.last().unwrap();
        match instr {
            Instr::Noop => reg_status.push(reg_val),
            Instr::Addx(val) => {
                reg_status.push(reg_val);
                reg_status.push(reg_val + val);
            }
        }
    }

    let mut sum = 0;
    let samples = [20, 60, 100, 140, 180, 220];
    for i in samples {
        let strength = reg_status.get(i).unwrap();
        let strength = i as i32 * strength;
        
        //println!("Curing Cycle {}: {}", i, strength);
        sum += strength;
    }

    // Print the answer to the first part
    println!("First Answer: {:?}", sum);

    // Print the answer to the second part
    let mut screen = vec!['.'; 241];

    for (i, reg) in reg_status.iter().enumerate() {
        //println!("Cycle {}: {}", i, reg);
        if i == 0 {
            // we ignore 0
            continue;
        }

        // check if "sprite" covers current pixel
        let x = (i as i32) % 40;
        if (x - reg) >=0 && (x - reg) < 3 {
            screen[i] = '#';
        }
    }

    let mut screen_str = String::new();
    for (i, c) in screen.iter().enumerate() {
        if i == 0 {
            // we ignore 0
            continue;
        }
        screen_str.push(*c);
        if i % 40 == 0 {
            screen_str.push('\n');
        }
    }
    println!("{}", screen_str);
}

fn main() {
    println!("Advent of Code, Day 10");

    part_1();
}

