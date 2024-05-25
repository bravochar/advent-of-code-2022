const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use core::cmp::Ordering;
use std::{fmt, vec};
use std::cmp::max;
use std::fs::read_to_string;
use std::io::{prelude::*, BufReader};
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct JetStream {
    filename: String,
    chars: Vec<char>
}

impl JetStream {
    fn new(filename: &str) -> JetStream {
        let buf = read_to_string(&filename).unwrap();
        let chars = buf.chars().rev().collect();

        JetStream{
            filename: filename.to_owned(),
            chars}
    }
}

impl <'v> Iterator for JetStream {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.chars.pop() {
            Some(c) => Some(c),
            _ => {
                let buf = read_to_string(&self.filename).unwrap();
                self.chars = buf.chars().rev().collect();
                Some(self.chars.pop().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_jet_stream() {
        let mut js = JetStream::new("./test");
        let test_chars = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        for c in test_chars.chars() {
            assert_eq!(c, js.next().unwrap());
        }
        for c in test_chars.chars() {
            assert_eq!(c, js.next().unwrap());
        }
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y:i32) -> Point {
        Point {x, y}
    }
}

struct Shaft {
    rows: HashMap<i32, Vec<Point>>,
    high_point: i32
}

const SHAFT_WIDTH: i32 = 7;
impl Shaft {
    fn new() -> Self {
        let rows = HashMap::new();
        Shaft { rows, high_point: 0 }
    }
}

impl fmt::Display for Shaft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rval = fmt::Result::Ok(());

        // TODO: account for when shape make highest point more than this
        let mut y = self.high_point + 3;
        while y >= 0 && rval.is_ok() {
            // TODO: make this a Vec<&Point> and add points from both
            //      the shaft and the falling piece
            //
            let mut row_points = match self.rows.get(&y){
                None => {
                    rval = write!(f, "|.......|\n");
                    y -= 1;
                    continue
                },
                Some(r) => r.clone()
            };
            row_points.sort();

            let mut row_str = "|".to_owned();
            let mut x = 0;
            for p in row_points.iter() {
                while x < p.x {
                    row_str.push('.');
                    x += 1;
                }
                row_str.push('#');
                x += 1;
            }
            while x < SHAFT_WIDTH {
                row_str.push('.');
                x += 1;
            }
            row_str.push('|');
            rval = write!(f, "{}\n", row_str);
            y -= 1;
        }
        if rval.is_ok() {
            rval = write!(f, "+-------+\n");
        }
        rval
    }
}

fn part_1(jets: JetStream) -> i32 {
    let mut shaft = Shaft::new();

    println!("{}", shaft);

    shaft.high_point
}

fn part_2() {
}

fn main() {
    println!("Advent of Code, Day 17");

    // read in the input
    let jets = JetStream::new(FILENAME);


    let now = Instant::now();
    use std::time::Instant;
    part_1(jets);
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);

    if false {
        let now = Instant::now();
        part_2();
        let elapsed = now.elapsed();
        println!("Took {:.5?}", elapsed);
    }
}
