const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_file(filename: &str) -> Vec<i32> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        rval.push(line.parse().unwrap());
    }
    rval
}

fn part_1(mut numbers: Vec<i32>) -> i32 {
    let orig = numbers.clone();
    let vec_len = numbers.len();

    let mut old_index: usize = 0;
    for (_, val) in orig.iter().enumerate() {
        if *val == 0 {
            continue;
        }
        // find val in numbers
        // TODO: be more clever?
        while old_index < vec_len {
            if numbers[old_index] == *val {
                break
            }
            old_index += 1;
        }

        // % makes it a positive integer
        numbers.remove(old_index);
        let modulo = vec_len as i32 - 1;
        let move_dist: i32 = val.to_owned().rem_euclid(modulo);
        let new_index = old_index as i32 + move_dist;
        let mut new_index = new_index % modulo;
        if new_index == 0 {
            new_index = modulo;
        }
        numbers.insert(new_index.try_into().unwrap(), val.to_owned());
        println!("{} moves from {} to {}", val, old_index, new_index);

        //println!("{:?}", numbers);
    }

    // find zero
    let mut zero_index = 0;
    for (i, val) in numbers.iter().enumerate() {
        if val == &0 {
            zero_index = i;
            break;
        }
    }

    println!("Found zero at {}", zero_index);
    let sum = numbers.get((zero_index + 1000) % vec_len).unwrap();
    let sum = sum + numbers.get((zero_index + 2000) % vec_len).unwrap();
    let sum = sum + numbers.get((zero_index + 3000) % vec_len).unwrap();

    sum
}

fn part_2(numbers: Vec<i32>) -> i64 {
    let mut numbers: Vec<(usize, i64)> = numbers.iter().enumerate()
            .map(|c| {(c.0, c.1.to_owned() as i64 * 811589153)}).collect();
    let vec_len = numbers.len();

    for _ in 0..10 {
        for i in 0..vec_len {
            let mut old_index: usize = 0;
            for (j, val) in numbers.iter().enumerate() {
                if val.0 == i {
                    old_index = j;
                    break;
                }
            }
            let cur = numbers.remove(old_index);
            let val = cur.1;
            //println!("Found {cur:?} at {old_index} (orig: {i})");

            if val == 0 {
                numbers.insert(old_index, cur);
                continue;
            }

            // % makes it a positive integer
            let modulo = vec_len as i64 - 1;
            let move_dist: i64 = val.rem_euclid(modulo);
            let new_index = old_index as i64 + move_dist;
            let mut new_index = new_index % modulo;
            if new_index == 0 {
                new_index = modulo;
            }

            numbers.insert(new_index.try_into().unwrap(), cur);
        }
        //println!("{:?}", numbers);
    }

    // find zero
    let mut zero_index = 0;
    for (i, val) in numbers.iter().enumerate() {
        if val.1 == 0 {
            zero_index = i;
            break;
        }
    }

    println!("Found zero at {}", zero_index);
    let sum = numbers.get((zero_index + 1000) % vec_len).unwrap().1;
    let sum = sum + numbers.get((zero_index + 2000) % vec_len).unwrap().1;
    let sum = sum + numbers.get((zero_index + 3000) % vec_len).unwrap().1;

    sum
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let numbers = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(numbers.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 3);
    } else {
        assert_eq!(answer, 2203);
    }

    let now = Instant::now();
    let answer = part_2(numbers.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 1623178306);
    }
}