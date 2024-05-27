//const FILENAME: &str = "./input";
const FILENAME: &str = "./test";

use std::{fmt, vec};
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Clone)]
struct JetStream {
    filename: String,
    chars: Vec<char>,
    cycle_len: i32,
    chars_left: i32,
}

impl JetStream {
    fn new(filename: &str) -> JetStream {
        let buf = read_to_string(&filename).unwrap();
        let chars: Vec<char> = buf.chars().rev().collect();
        let cycle_len = chars.len() as i32;

        JetStream{
            filename: filename.to_owned(),
            chars,
            cycle_len,
            chars_left: cycle_len}
    }
}

impl <'v> Iterator for JetStream {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.chars.pop() {
            Some(c) => {
                self.chars_left -= 1;
                Some(c)},
            _ => {
                let buf = read_to_string(&self.filename).unwrap();
                self.chars = buf.chars().rev().collect();
                self.chars_left = self.cycle_len - 1;
                Some(self.chars.pop().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_test_shaft() -> Shaft {
        let jets = JetStream::new("./test");
        Shaft::new(jets)
    }

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
    
    #[test]
    fn test_first_shape() {
        let mut shaft = get_test_shaft();

        shaft.add_next_shape();
        shaft.drop_shape();
        let shaft_str = format!("{}", shaft);
        let test_str = "|..####.|\n\
                              +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);
    }
    
    #[test]
    fn test_second_shape() {
        let mut shaft = get_test_shaft();

        for _ in 0..2 {
            shaft.add_next_shape();
            shaft.drop_shape();
        }
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...#...|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);
    }
    
    #[test]
    fn test_third_shape() {
        let mut shaft = get_test_shaft();

        for _ in 0..3 {
            shaft.add_next_shape();
            shaft.drop_shape();
        }
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..#....|\n\
        |..#....|\n\
        |####...|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);
    }

    #[test]
    fn test_09_shapes() {
        let mut shaft = get_test_shaft();

        for _ in 0..9 {
            shaft.add_next_shape();
            shaft.drop_shape();
        }
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |....#..|\n\
        |....#..|\n\
        |....##.|\n\
        |....##.|\n\
        |..####.|\n\
        |.###...|\n\
        |..#....|\n\
        |.####..|\n\
        |....##.|\n\
        |....##.|\n\
        |....#..|\n\
        |..#.#..|\n\
        |..#.#..|\n\
        |#####..|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);
    }

    #[test]
    fn test_10_shapes() {
        let mut shaft = get_test_shaft();

        for _ in 0..10 {
            shaft.add_next_shape();
            shaft.drop_shape();
        }
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |....#..|\n\
        |....#..|\n\
        |....##.|\n\
        |##..##.|\n\
        |######.|\n\
        |.###...|\n\
        |..#....|\n\
        |.####..|\n\
        |....##.|\n\
        |....##.|\n\
        |....#..|\n\
        |..#.#..|\n\
        |..#.#..|\n\
        |#####..|\n\
        |..###..|\n\
        |...#...|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);
    }
}

#[derive(Debug)]
struct Shape {
    rows: HashMap<i64, Vec<i64>>,
}

impl Shape {
    /*
     * Each rock appears so that its left edge is two units away from the left
     * wall and its bottom edge is three units above the highest rock in the 
     * room (or the floor, if there isn't one).
     */
    fn horizontal_line(high_point: i64) -> Shape {
        /*
         * ####
         */
        let base_y = high_point + 4;
        let mut rows = HashMap::new();
        rows.insert(base_y, vec![2, 3, 4, 5]);

        Shape{rows}
    }

    fn cross(high_point: i64) -> Shape {
        /*
         * .#.
         * ###
         * .#.
         */
        let base_y = high_point + 4;
        let mut rows = HashMap::new();
        rows.insert(base_y, vec![3]);
        rows.insert(base_y + 1, vec![2, 3, 4]);
        rows.insert(base_y + 2, vec![3]);

        Shape{rows}
    }

    fn ell(high_point: i64) -> Shape {
        /*
         * ..#
         * ..#
         * ###
         */
        let base_y = high_point + 4;
        let mut rows = HashMap::new();
        rows.insert(base_y, vec![2, 3, 4]);
        rows.insert(base_y + 1, vec![4]);
        rows.insert(base_y + 2, vec![4]);

        Shape{rows}
    }

    fn vertical_line(high_point: i64) -> Shape {
        /*
         * ####
         */
        let base_y = high_point + 4;
        let mut rows = HashMap::new();
        rows.insert(base_y, vec![2]);
        rows.insert(base_y + 1, vec![2]);
        rows.insert(base_y + 2, vec![2]);
        rows.insert(base_y + 3, vec![2]);

        Shape{rows}
    }

    fn square(high_point: i64) -> Shape {
        /*
         * ####
         */
        let base_y = high_point + 4;
        let mut rows = HashMap::new();
        rows.insert(base_y, vec![2, 3]);
        rows.insert(base_y + 1, vec![2, 3]);

        Shape{rows}
    }

    fn high_point(&self) -> i64 {
        self.rows.keys().max().unwrap().to_owned()
    }
}

struct Shaft {
    jets: JetStream,
    rows: HashMap<i64, Vec<i64>>,
    high_point: i64,
    shape: Option<Shape>,
    next_shape: i64,
    shape_count: i64,
    left_closure: i64,
    right_closure: i64,
}

const SHAFT_WIDTH: i64 = 7;
impl Shaft {
    fn new(jets: JetStream) -> Self {
        let rows = HashMap::new();
        Shaft {
            jets,
            rows,
            high_point: -1,
            shape: None,
            next_shape: 0,
            shape_count: 0,
            left_closure: 0,
            right_closure: 0,
        }

    }

    fn add_shape(&mut self, s: Shape) {
        match self.shape {
            None => self.shape = Some(s),
            _ => panic!("Cannot add another shape")
        }
    }

    fn add_next_shape(&mut self) {
        match self.next_shape {
            0 => {
                if self.jets.chars_left == 0 {
                    println!("Jetstream looped after {} shapes", self.shape_count);
                }
                self.add_shape(Shape::horizontal_line(self.high_point))
            },
            1 => self.add_shape(Shape::cross(self.high_point)),
            2 => self.add_shape(Shape::ell(self.high_point)),
            3 => self.add_shape(Shape::vertical_line(self.high_point)),
            _ => self.add_shape(Shape::square(self.high_point))
        }
        self.next_shape = (self.next_shape + 1) % 5;
        self.shape_count += 1;

        if self.shape_count % 8096 == 0 {
            self.prune_dead_rows();
        }
    }

    fn move_right(&mut self) {
        let s = self.shape.as_mut().unwrap();

        /*
         * ensure that all points can move right without:
         *  1) moving beyond the wall at SHAFT_WIDTH
         *  2) colliding with existing rock
         * 
         *  TODO: we only need to check for free space to the RIGHT of the 
         *      right-most point in each row, and then "move" the left-most
         *      point into that free space
         */
        let mut new_rows = HashMap::new();
        for (y, shape_points) in s.rows.iter() {
            // get min and max value
            let mut new_row = shape_points.clone();
            if true {
                let min_x = shape_points.iter().min().unwrap();
                let max_x = shape_points.iter().max().unwrap();
                let new_x = max_x + 1;
                if new_x >= SHAFT_WIDTH {
                    return;
                }
                match self.rows.get(y) {
                    Some(r) => {
                        if r.contains(&(max_x + 1)) {
                            return;
                        }
                    },
                    _ => ()
                }
                new_row.retain(|x| { x != min_x});
                new_row.push(max_x + 1);
            } else {
                new_row = shape_points.iter().map(|x| {x+1}).collect();
                let new_max = new_row.iter().max().unwrap();
                if new_max >= &SHAFT_WIDTH {
                    return;
                }
                match self.rows.get(y) {
                    Some(r) => {
                        if r.contains(&new_max) {
                            return;
                        }
                    },
                    _ => ()
                }
            }
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
         * 
         *  TODO: we only need to check for free space to the LEFT of the 
         *      left-most point in each row, and then "move" the right-most
         *      point into that free space
         */
        let mut new_rows = HashMap::new();
        for (y, shape_points) in s.rows.iter() {
            let mut new_row = Vec::new();
            for x in shape_points.iter() {
                let new_x = x - 1;
                if new_x < 0 {
                    return;
                }
                match self.rows.get(y) {
                    Some(rock_points) => {
                        if rock_points.contains(&new_x) {
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
        } else if jet_char == '<' {
            self.move_left();
        } else {
            panic!("BAD CHAR {}", jet_char);
        }
    }

    fn prune_dead_rows(&mut self) {
        let new_floor = if self.left_closure < self.right_closure {
            self.left_closure
        } else {
            self.right_closure
        };
        
        self.rows.retain( |k, _| { k >= &new_floor });
    }

    fn petrify_shape(&mut self) {
        let s = self.shape.as_mut().unwrap();
        let mut new_floor = false;

        // update high point
        let shape_high_point = s.rows.keys().max().unwrap().to_owned();
        if shape_high_point > self.high_point {
            self.high_point = shape_high_point;
        }

        for (y, shape_points) in s.rows.iter() {
            let mut row = match self.rows.remove(y) {
                Some(r) => r,
                None => Vec::new()
            };
            for x in shape_points.into_iter() {
                row.push(x.clone());
            }
            if row.contains(&0) {
                self.left_closure = y.to_owned();
                new_floor = true;

            } else if row.contains(&(SHAFT_WIDTH - 1)) {
                self.right_closure = y.to_owned();
                new_floor = true;
            }
            self.rows.insert(y.clone(), row);
        }

        self.shape = None;

        /*
        if new_floor {
            self.prune_dead_rows();
        }
         */
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
        let rval = self.gravity_turn();
        rval
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
            None => self.high_point,
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

fn part_1(jets: JetStream) -> i64 {
    let mut shaft = Shaft::new(jets);
    
    for _ in 0..2022 {
        shaft.add_next_shape();
        shaft.drop_shape();
    }

    if FILENAME == "./test" {
        assert_eq!(shaft.high_point + 1, 3068);
    }

    shaft.high_point + 1
}

fn part_2(jets: JetStream) -> i64 {
    /*
     * This is 1-trillion rocks, so it is not feasible to run the simultation
     * for the 120 days it would take to calculate :-|
     */
    // 1,000,000,000,000;
    let limit: i64 = 1000000000000;
    let cycle_len = 1000000;

    let mut shaft = Shaft::new(jets);

    /*
     * with 5 different shapes and
     */
    let mut prev_high = 0;
    for i in 0..1 {
        for _ in 0..cycle_len {
            shaft.add_next_shape();
            shaft.drop_shape();
        }
        println!("Cycle {}: {} (added {})",
            i + 1,
            shaft.high_point,
            shaft.high_point - prev_high);
        prev_high = shaft.high_point;
    }

    shaft.high_point + 1
}

fn main() {
    println!("Advent of Code, Day 17");

    // read in the input
    let jets = JetStream::new(FILENAME);


    let now = Instant::now();
    use std::time::Instant;
    println!("Part 1: {}", part_1(jets.clone()));
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);

    let now = Instant::now();
    println!("Part 2: {}", part_2(jets));
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);
}
