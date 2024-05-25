const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use core::cmp::Ordering;
use std::hash::Hash;
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

#[derive(Debug)]
struct Shape {
    rows: HashMap<i32, Vec<i32>>,
}

impl Shape {
    /*
     * Each rock appears so that its left edge is two units away from the left
     * wall and its bottom edge is three units above the highest rock in the 
     * room (or the floor, if there isn't one).
     */
    fn horizontal_line(high_point: i32) -> Shape {
        /*
         * ####
         */
        let base_y = high_point + 3;
        let mut rows = HashMap::new();
        let mut row = Vec::new();
        for x in 2..6 {
            row.push(x);
        }
        rows.insert(base_y, row);

        Shape{rows}
    }

    fn high_point(&self) -> i32 {
        self.rows.keys().max().unwrap().to_owned()
    }
}

struct Shaft {
    jets: JetStream,
    rows: HashMap<i32, Vec<i32>>,
    high_point: i32,
    shape: Option<Shape>
}

const SHAFT_WIDTH: i32 = 7;
impl Shaft {
    fn new(jets: JetStream) -> Self {
        let rows = HashMap::new();
        Shaft {
            jets,
            rows,
            high_point: 0,
            shape: None }
    }

    fn add_shape(&mut self, s: Shape) {
        match self.shape {
            None => self.shape = Some(s),
            _ => panic!("Cannot add another shape")
        }
    }

    fn move_right(&mut self) {
        let s = self.shape.as_mut().unwrap();

        /*
         * ensure that all points can move right without:
         *  1) moving beyond the wall at SHAFT_WIDTH
         *  2) colliding with existing rock
         */
        let mut new_rows = HashMap::new();
        for (y, shape_points) in s.rows.iter() {
            let mut new_row = Vec::new();
            for x in shape_points.iter() {
                let new_x = x + 1;
                if new_x >= SHAFT_WIDTH {
                    println!("Collision with wall - not moving");
                    return;
                }
                match self.rows.get(y) {
                    Some(rock_points) => {
                        if rock_points.contains(&new_x) {
                            println!("Collision with rock - not moving");
                            return;
                        }
                    },
                    _ => ()
                }
                new_row.push(new_x);
            };
            new_rows.insert(y.to_owned(), new_row);
        }
        s.rows = new_rows;
    }

    fn move_left(&mut self) {
        let s = self.shape.as_mut().unwrap();

        /*
         * ensure that all points can move right without:
         *  1) moving beyond the wall at SHAFT_WIDTH
         *  2) colliding with existing rock
         */
        let mut new_rows = HashMap::new();
        for (y, shape_points) in s.rows.iter() {
            let mut new_row = Vec::new();
            for x in shape_points.iter() {
                let new_x = x - 1;
                if new_x < 0 {
                    println!("Collision with wall - not moving");
                    return;
                }
                match self.rows.get(y) {
                    Some(rock_points) => {
                        if rock_points.contains(&new_x) {
                            println!("Collision with rock - not moving");
                            return;
                        }
                    },
                    _ => ()
                }
                new_row.push(new_x);
            };
            new_rows.insert(y.to_owned(), new_row);
        }
        s.rows = new_rows;
    }

    fn jet_turn(&mut self) {
        let jet_char = self.jets.next().unwrap();

        if jet_char == '>' {
            self.move_right();
        } else {
            self.move_left();
        }
    }

    fn petrify_shape(&mut self) {
        let s = self.shape.as_mut().unwrap();
        for (y, shape_points) in s.rows.iter() {
            let mut row = match self.rows.remove(y) {
                Some(r) => r,
                None => Vec::new()
            };
            for x in shape_points.into_iter() {
                row.push(x.clone());
            }
            self.rows.insert(y.clone(), row);
        }

        self.shape = None;
    }

    fn gravity_turn(&mut self) -> bool {
        let s = self.shape.as_mut().unwrap();

        /*
         * ensure that all points can move right without:
         *  1) moving beyond the wall at SHAFT_WIDTH
         *  2) colliding with existing rock
         */
        let mut new_rows = HashMap::new();
        for (y, shape_points) in s.rows.iter() {
            let mut new_row = Vec::new();
            let new_y = y - 1;
            if new_y < 0 {
                self.petrify_shape();
                return true;
            }
            for x in shape_points.iter() {
                match self.rows.get(&new_y) {
                    Some(rock_points) => {
                        if rock_points.contains(&x) {
                            println!("Collision with rock - not falling");
                            self.petrify_shape();
                            return true;
                        }
                    },
                    _ => ()
                }
                new_row.push(x.to_owned());
            };
            new_rows.insert(new_y.to_owned(), new_row);
        }
        s.rows = new_rows;
        
        return false
    }

    fn full_turn(&mut self) -> bool {
        self.jet_turn();
        self.gravity_turn()
    }

    fn drop_shape(&mut self) {
        while !self.full_turn() {()}
    }
}

impl fmt::Display for Shaft {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rval = fmt::Result::Ok(());

        // TODO: account for when shape make highest point more than this
        let mut y = match self.shape {
            None => self.high_point + 3,
            Some(_) => self.shape.as_ref().unwrap().high_point()
        };

        while y >= 0 && rval.is_ok() {
            // TODO: make this a Vec<&Point> and add points from both
            //      the shaft and the falling piece
            //
            let rock_points = self.rows.get(&y);
            let shape_points = match self.shape.as_ref() {
                Some(s) => {
                    s.rows.get(&y)
                },
                None => None
            };

            let mut row_str = "|".to_owned();
            for x in 0..SHAFT_WIDTH {
                match shape_points {
                    Some(v) => {
                        if v.contains(&x) {
                            row_str.push('@');
                            continue;
                        }
                    },
                    _ => ()
                }
                match rock_points {
                    Some(v) => {
                        if v.contains(&x) {
                            row_str.push('#');
                            continue;
                        }
                    },
                    _ => ()
                }
                row_str.push('.');
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
    let mut shaft = Shaft::new(jets);

    shaft.add_shape(Shape::horizontal_line(shaft.high_point));
    println!("{}", shaft);
    shaft.jet_turn();
    println!("{}", shaft);
    shaft.gravity_turn();
    println!("{}", shaft);
    
    shaft.full_turn();
    println!("{}", shaft);
    
    shaft.full_turn();
    println!("{}", shaft);
    
    shaft.full_turn();
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
