use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::hash::Hash;
use std::collections::HashSet;

const FILENAME: &str = "./input";

#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn move_head(mut head: Coord, dir: &str, mag: i32) -> Coord {
    match dir {
        "R" => head.x += mag,
        "L" => head.x -= mag,
        "U" => head.y += mag,
        "D" => head.y -= mag,
        _ => println!("ERROR: invalid move '{}'", dir),
    }
    head
}

fn follow(tail: &mut Coord, head: &Coord) {
    let x_diff: i32 = head.x - tail.x;
    let y_diff: i32 = head.y - tail.y;

    if x_diff.abs() == 2 {
        tail.x += x_diff / 2;
        if y_diff.abs() > 0 {
            tail.y += y_diff / y_diff.abs();
        }

    } else if y_diff.abs() == 2 {
        tail.y += y_diff / 2;
        if x_diff.abs() > 0 {
            tail.x += x_diff / x_diff.abs();
        }
    } else if y_diff.abs() > 2 || x_diff.abs() > 2 {
        panic!("FATAL ERROR: Coords are too distant: {:?} {:?}", tail, head);
    }
}

fn move_from_line(line_str: String) -> (String, i32) {
    let splits: Vec<&str> = line_str.split(' ').collect();

    if splits.len() == 2 {
        let dir = splits[0];
        let mag: i32 = splits[1].parse().expect("Couldn't parse move magnitude");

        (String::from(dir), mag)
    } else {
        println!("ERROR: Line was not valid '{}'", line_str);
        (String::from("X"), 0)
    }
}


fn dedup<T: Eq + Hash + Copy>(v: &mut Vec<T>) { // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(*e));
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut head = Coord {
        x: 0,
        y: 0,
    };
    let mut tail = Coord {
        x: 0,
        y: 0,
    };
    let mut tail_pos = Vec::<Coord>::new();

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let (dir, mag) = move_from_line(line_str);

        for _ in 0..mag {
            head = move_head(head, &dir, 1);
            follow(&mut tail, &head);
            tail_pos.push(Coord{
                x: tail.x,
                y: tail.y
            })
        }
    }

    // Print the answer to the first part
    dedup(&mut tail_pos);
    let answer = tail_pos.len();
    println!("First Answer: {:?}", answer);
}


fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // init state for answer
    let mut head = Coord {
        x: 0,
        y: 0,
    };

    // make this a vector of 9 tails
    let mut tails: Vec<Coord> = Vec::new();
    for _ in 0..9 {
        tails.push(Coord {
            x: 0,
            y: 0,
        });
    }
    let mut tail_pos = Vec::<Coord>::new();

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let (dir, mag) = move_from_line(line_str);

        for _ in 0..mag {
            head = move_head(head, &dir, 1);

            let mut prev: &Coord = &head;
            for tail in tails.iter_mut() {
                follow(tail, prev);
                prev = tail;
            }
            tail_pos.push(Coord{
                x: tails[8].x,
                y: tails[8].y
            });
        }
    }

    // Print the answer to the second part
    dedup(&mut tail_pos);
    let answer = tail_pos.len();
    println!("Second Answer: {:?}", answer);
}

fn main() {
    println!("Advent of Code, Day 9");

    part_1();
    part_2();
}

