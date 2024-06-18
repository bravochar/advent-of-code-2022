use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";

fn crates_from_str(l: &str) -> Vec::<char> {
    let mut rval = Vec::<char>::new();
    let char_vec: Vec::<char> = l.chars().collect();

    let mut i = 1;

    // with input of the form:
    //     0123456
    //     -------
    //         [D]    
    //     [N] [C]    
    //
    // We can start at index 1, push the value onto the return vector,
    // increment the index by 4, and continue until the index exceeds the 
    // bounds of the string. Spaces will signify nothing in the stack at
    // that index.
    while i < char_vec.len() {
        rval.push(char_vec[i]);
        i += 4;
    }

    rval
}

fn moves_from_str(l: &str) -> (usize, usize, usize) {
    let l_vec: Vec::<&str> = l.split(' ').collect();

    if l_vec.len() != 6 {
        return( 0, 0, 0);
    }

    let num = l_vec[1].parse().expect("Could not parse number of crates to move");
    let src = l_vec[3].parse().expect("Could not parse source of crates");
    let dst = l_vec[5].parse().expect("Could not parse dest. of crates");

    (num, src, dst)
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut answer = String::new();
    let mut building_stacks = true;
    let mut stacks = Vec::<Vec::<char>>::new();

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        if building_stacks {
            // check for end of initial state
            if line_str.starts_with(" 1 ") {
                building_stacks = false;

                // print starting point
                for (i, stack) in stacks.iter().enumerate() {
                    println!("{}: {:?}", i, stack);
                }

                continue;
            }
            
            // split line into crates at this layer
            let crates = crates_from_str(&line_str);

            // add this layer of crates to stacks, inserting at "bottom"
            for (i, c) in crates.iter().enumerate() {
                // add stack if not present
                while stacks.len() <= i {
                    stacks.push(Vec::<char>::new());
                }

                if *c != ' ' {
                    stacks[i].insert(0, *c);
                }
            }

        } else {
            if !line_str.starts_with("move") {
                continue;
            }
            let (num, src, dst) = moves_from_str(&line_str);

            // move crates according to move
            for _ in 0..num {
                let c = stacks[src - 1].pop().expect("Stack was empty?!");

                stacks[dst - 1].push(c);
            }
        }
    }

    for stack in stacks.iter() {
        answer.push(
            stack.last().copied().expect("Couldn't get last crate from stack"));
    }

    // Print the answer to the first part
    println!("First Answer: {:?}", answer);
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut answer = String::new();
    let mut building_stacks = true;
    let mut stacks = Vec::<Vec::<char>>::new();

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");

        if building_stacks {
            // check for end of initial state
            if line_str.starts_with(" 1 ") {
                building_stacks = false;

                // print starting point
                for (i, stack) in stacks.iter().enumerate() {
                    println!("{}: {:?}", i, stack);
                }

                continue;
            }
            
            // split line into crates at this layer
            let crates = crates_from_str(&line_str);

            // add this layer of crates to stacks, inserting at "bottom"
            for (i, c) in crates.iter().enumerate() {
                // add stack if not present
                while stacks.len() <= i {
                    stacks.push(Vec::<char>::new());
                }

                if *c != ' ' {
                    stacks[i].insert(0, *c);
                }
            }

        } else {
            if !line_str.starts_with("move") {
                continue;
            }
            let (num, src, dst) = moves_from_str(&line_str);

            // move crates according to move, preserving order
            let mut to_move = Vec::<char>::new();
            for _ in 0..num {
                let c = stacks[src - 1].pop().expect("Stack was empty?!");

                to_move.insert(0, c);
            }

            for c in to_move.iter() {
                stacks[dst - 1].push(*c);
            }
        }
    }

    for stack in stacks.iter() {
        answer.push(
            stack.last().copied().expect("Couldn't get last crate from stack"));
    }

    // Print the answer to the second part
    println!("Second Answer: {:?}", answer);
}

fn main() {
    println!("Advent of Code, Day 4");

    part_1();
    part_2();
}

