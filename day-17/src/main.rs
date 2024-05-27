const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::hash::Hash;
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

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        |.......|\n\
        |.......|\n\
        |.......|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        |.......|\n\
        |.......|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        |.......|\n\
        |.......|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        |.......|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        |.......|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.gravity_turn();

        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@@@@|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..@@@@.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);

        assert_eq!(shaft.high_point, 0);
        assert_eq!(shaft.high_point, 0);
    }
    
    #[test]
    fn test_second_shape() {
        let mut shaft = get_test_shaft();

        shaft.add_next_shape();
        shaft.drop_shape();
        shaft.add_next_shape();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@...|\n\
        |..@@@..|\n\
        |...@...|\n\
        |.......|\n\
        |.......|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..@....|\n\
        |.@@@...|\n\
        |..@....|\n\
        |.......|\n\
        |.......|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);

        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..@....|\n\
        |.@@@...|\n\
        |..@....|\n\
        |.......|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@...|\n\
        |..@@@..|\n\
        |...@...|\n\
        |.......|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);
        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@...|\n\
        |..@@@..|\n\
        |...@...|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..@....|\n\
        |.@@@...|\n\
        |..@....|\n\
        |.......|\n\
        |..####.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);
        shaft.gravity_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |..@....|\n\
        |.@@@...|\n\
        |..@....|\n\
        |..####.|\n\
        +-------+\n";
        println!("{}", shaft_str);
        println!("{}", test_str);
        assert_eq!(shaft_str, test_str);

        shaft.jet_turn();
        let shaft_str = format!("{}", shaft);
        let test_str = "\
        |...@...|\n\
        |..@@@..|\n\
        |...@...|\n\
        |..####.|\n\
        +-------+\n";
        assert_eq!(shaft_str, test_str);
        shaft.gravity_turn();
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

        assert_eq!(shaft.high_point, 3);
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
        assert_eq!(shaft.high_point, 5);
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
        assert_eq!(shaft.high_point, 16);
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
        assert_eq!(shaft.high_point, 16);
    }

    #[test]
    fn test_100_shapes() {
        let mut shaft = get_test_shaft();

        let first_100: Vec<i64> = vec![
            1, 4, 6, 7, 9, 10, 13, 15, 17, 17,
            18, 21, 23, 23, 25, 26, 29, 32, 36, 36,
            37, 39, 42, 42, 43, 44, 47, 49, 51, 51,
            51, 53, 56, 60, 60, 61, 63, 64, 66, 66,
            67, 69, 70, 72, 72, 73, 76, 78, 78, 78,
            79, 82, 85, 89, 89, 90, 92, 95, 95, 96,
            97, 100, 102, 104, 104, 104, 106, 109, 113, 113,
            114, 116, 117, 119, 119, 120, 122, 123, 125, 125,
            126, 129, 131, 131, 131, 132, 135, 138, 142, 142,
            143, 145, 148, 148, 149, 150, 153, 155, 157, 157,
        ];

        for (i, score) in first_100.into_iter().enumerate() {
            shaft.add_next_shape();
            shaft.drop_shape();

            if shaft.high_point + 1 != score {
                println!("{}: Expected {}: found {}",
                        i, score, shaft.high_point + 1);
                println!("{}", shaft);
                assert_eq!(shaft.high_point + 1, score);
            }
        }


    }
}

#[derive(Debug)]
struct Shape {
    x: i32,
    width: i32,
    y: i64,
    rows: Vec<Vec<i32>>,
}

impl Shape {
    /*
     * Each rock appears so that its left edge is two units away from the left
     * wall and its bottom edge is three units above the highest rock in the 
     * room (or the floor, if there isn't one).
     */
    fn new(high_point: i64, rows: Vec<Vec<i32>>) -> Shape {
        Shape {
            x: 2,
            width: rows.iter().map(|r|{
                r.len()
            }).max().unwrap() as i32,
            y: high_point + 3 + 1,
            rows
        }
    }

    fn horizontal_line(high_point: i64) -> Shape {
        /*
         * ####
         */
        let mut rows = Vec::new();
        rows.push( vec![0, 1, 2, 3]);

        Shape::new(high_point, rows)
    }

    fn cross(high_point: i64) -> Shape {
        /*
         * .#.
         * ###
         * .#.
         */
        let mut rows = Vec::new();
        rows.push(vec![   1]);
        rows.push(vec![0, 1, 2]);
        rows.push(vec![   1]);

        Shape::new(high_point, rows)
    }

    fn ell(high_point: i64) -> Shape {
        /*
         * ..#
         * ..#
         * ###
         */
        let mut rows = Vec::new();
        rows.push(vec![0, 1, 2]);
        rows.push(vec![      2]);
        rows.push(vec![      2]);

        Shape::new(high_point, rows)
    }

    fn vertical_line(high_point: i64) -> Shape {
        /*
         * ####
         */
        let mut rows = Vec::new();
        rows.push(vec![0]);
        rows.push(vec![0]);
        rows.push(vec![0]);
        rows.push(vec![0]);

        Shape::new(high_point, rows)
    }

    fn square(high_point: i64) -> Shape {
        /*
         * ####
         */
        let mut rows = Vec::new();
        rows.push(vec![0, 1]);
        rows.push(vec![0, 1]);

        Shape::new(high_point, rows)
    }

    fn high_point(&self) -> i64 {
        self.y + self.rows.len() as i64 - 1
    }
}

struct Shaft {
    jets: JetStream,
    rows: HashMap<i64, Vec<i32>>,
    high_point: i64,
    shape: Option<Shape>,
    next_shape: i64,
    shape_count: i64,
    left_closure: i64,
    right_closure: i64,
    cycle_detector: [HashMap<i32, i64>; 5],
    cycle_start: i64,
    cycle_len: i64,
}

const SHAFT_WIDTH: i32 = 7;
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
            cycle_detector: [
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new(),
                HashMap::new()],
            cycle_start: 0,
            cycle_len: 0,
        }

    }

    fn add_shape(&mut self, s: Shape) {
        match self.shape {
            None => self.shape = Some(s),
            _ => panic!("Cannot add another shape")
        }
    }

    fn detect_cycle(&mut self) {
        /*
         * Find the CYCLE. At some point, we will generate the SAME shape, at
         * the SAME point in the jetstream. From that point onward (or maybe
         * after 2 or 3 cycles), the added height from cycle to cycle (and
         * within cycles will be consistent). We need to create a hashset of
         * the jetstream position for when each shape is generated. When we find
         * a duplicate we have found a cycle?
         */
        match self.next_shape {
            0 => {
                if self.cycle_detector[0].contains_key(&self.jets.chars_left) {
                    if self.cycle_start == 0 {
                        self.cycle_start = self.shape_count;
                        /*
                        println!("Found cycle start: {}: {} jet position, score: {}",
                            self.shape_count,
                            self.jets.chars_left,
                            self.high_point);
                         */

                        self.cycle_detector[0] = HashMap::new();

                    } else if self.cycle_len == 0 {
                        self.cycle_len = self.shape_count - self.cycle_start;
                        /*
                        println!("Found cycle len {}: {} shapes, {} jet position, score: {}",
                            self.cycle_len,
                            self.shape_count,
                            self.jets.chars_left,
                            self.high_point);
                         */
                    }
                }
                self.cycle_detector[0].insert(self.jets.chars_left, self.shape_count);
            },
            _ => ()
        }
    }

    fn check_cycle(&mut self) {
        if self.cycle_start == 0 || self.cycle_len == 0{
            self.detect_cycle();
        }
    }

    fn add_next_shape(&mut self) {
        /*
         * Find the CYCLE. At some point, we will generate the SAME shape, at
         * the SAME point in the jetstream. From that point onward (or maybe
         * after 2 or 3 cycles), the added height from cycle to cycle (and
         * within cycles will be consistent). We need to create a hashset of
         * the jetstream position for when each shape is generated. When we find
         * a duplicate we have found a cycle?
         */
        self.check_cycle();
        match self.next_shape {
            0 => {
                self.add_shape(Shape::horizontal_line(self.high_point))
            },
            1 => {
                self.add_shape(Shape::cross(self.high_point))
            },
            2 => {
                self.add_shape(Shape::ell(self.high_point))
            },
            3 => {
                self.add_shape(Shape::vertical_line(self.high_point))
            },
            4 => {
                self.add_shape(Shape::square(self.high_point))
            },
            _ => panic!("Illegal shape {}", self.next_shape)
        }
        self.next_shape = (self.next_shape + 1) % 5;
        self.shape_count += 1;

        if self.cycle_len != 0 && self.shape_count % self.cycle_len == self.cycle_start {
            println!("New cycle {}: height: {}, shapes: {}, jets {}",
                self.shape_count / self.cycle_len,
                self.high_point,
                self.shape_count,
            self.jets.chars_left);
        }
    }

    fn move_right(&mut self) {
        let s = self.shape.as_mut().unwrap();

        let new_offset = s.x + 1;
        if new_offset + s.width > SHAFT_WIDTH {
            /*
            println!("Collision with wall: right");
            println!("{}", self);
             */
            return;
        }
        for (i, shape_points) in s.rows.iter().enumerate() {
            // check against rock formation in shaft
            let y = s.y + i as i64;
            match self.rows.get(&y) {
                Some(r) => {
                    if shape_points.iter().any(|x| {
                        r.contains(&(x + new_offset))
                    }) {
                        /*
                        println!("Collision with shape: right");
                        println!("{}", self);
                         */
                        return;}
                },
                _ => ()
            }
        }

        s.x = new_offset;
    }

    fn move_left(&mut self) {
        let s = self.shape.as_mut().unwrap();

        let new_offset = s.x - 1;
        if new_offset < 0 {
            /*
            println!("Collision with wall: left");
            println!("{}", self);
             */
            return;
        }
        for (i, shape_points) in s.rows.iter().enumerate() {
            // check against rock formation in shaft
            let y = s.y + i as i64;
            match self.rows.get(&y) {
                Some(r) => {
                    if shape_points.iter().any(|x| {
                        r.contains(&(x + new_offset))
                    }) {
                        /*
                        println!("Collision with shape: left");
                        println!("{}", self);
                         */
                        return;
                    }
                },
                _ => ()
            }
        }

        s.x -= 1;
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

        // update high point
        if s.high_point() > self.high_point {
            self.high_point = s.high_point();
        }

        for (i, shape_points) in s.rows.iter().enumerate() {
            let y = i as i64 + s.y;
            let mut row = match self.rows.remove(&y) {
                Some(r) => r,
                None => Vec::new()
            };
            for offset in shape_points.into_iter() {
                row.push(s.x + offset);
            }
            if row.contains(&0) {
                self.left_closure = y.to_owned();

            } else if row.contains(&(SHAFT_WIDTH - 1)) {
                self.right_closure = y.to_owned();
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
        let new_y = s.y - 1;
        if new_y <= self.high_point {
            for (i, shape_points) in s.rows.iter().enumerate() {
                let cur_y = new_y + i as i64;
                // TODO: row ZERO of shaft should be filled to remove this check
                if cur_y < 0 {
                    self.petrify_shape();
                    return true;
                }
                for offset in shape_points.iter() {
                    match self.rows.get(&cur_y) {
                        Some(rock_points) => {
                            if rock_points.contains(&(s.x + offset)) {
                                self.petrify_shape();
                                return true;
                            }
                        },
                        _ => ()
                    }
                };
            }
        }
        
        s.y -= 1;
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
            let mut shape_points: Option<Vec<i32>> = None;
            match self.shape.as_ref() {
                Some(s) => {
                    if y >= s.y {
                        let row_offset: usize = (y - s.y).try_into().unwrap();
                        if row_offset < s.rows.len() {
                            let offsets = s.rows.get(row_offset).unwrap();
                            shape_points = Some(
                                offsets.iter().map(|o| {
                                    o + s.x
                                }).collect());
                        }
                    }
                },
                None => ()
            };

            let mut row_str = "|".to_owned();
            for x in 0..SHAFT_WIDTH {
                match shape_points {
                    Some(ref v) => {
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

    shaft.high_point + 1
}

fn part_2(jets: JetStream) -> i64 {
    /*
     * This is 1-trillion rocks, so it is not feasible to run the simultation
     * for the 120 days it would take to calculate :-|
     */
    // 1,000,000,000,000;
    let limit: i64 = 1000000000000;

    let mut shaft = Shaft::new(jets);

    let mut prev_high = 0;
    while shaft.cycle_len == 0  {
            shaft.add_next_shape();
            shaft.drop_shape();

        if shaft.shape_count > 10000 {
            panic!();
        }
    }
    let shapes_to_start = shaft.shape_count - 1;

    for _ in 0..shaft.cycle_len-1 {
        shaft.add_next_shape();
        shaft.drop_shape();
    }

    // build LUT for height added during cycle
    let mut added_heights = Vec::new();
    prev_high = shaft.high_point;
    for _ in 0..shaft.cycle_len {
        shaft.add_next_shape();
        shaft.drop_shape();
        added_heights.push(shaft.high_point - prev_high);
    }
    let cycle_height = shaft.high_point - prev_high;
    let rval = shaft.high_point - cycle_height * 2;

    // the total height aft
    let mut added_heights_2 = Vec::new();
    prev_high = shaft.high_point;
    for _ in 0..shaft.cycle_len {
        shaft.add_next_shape();
        shaft.drop_shape();
        added_heights_2.push(shaft.high_point - prev_high);
    }

    assert_eq!(added_heights[..10], added_heights_2[..10]);

    let shapes_to_go = limit - shapes_to_start;
    let num_cycles = shapes_to_go / shaft.cycle_len;
    let rval = rval + num_cycles * cycle_height;

    let i: usize = (shapes_to_go % shaft.cycle_len).try_into().unwrap();
    rval + added_heights.get(i).unwrap()
}

fn main() {
    println!("Advent of Code, Day 17");

    // read in the input
    let jets = JetStream::new(FILENAME);


    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(jets.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 3068);
    }

    let now = Instant::now();
    let answer = part_2(jets.clone());
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);
    println!("Part 2: {}", answer);

    if FILENAME == "./test" {
        assert_eq!(answer, 1514285714288);
    } else {
        assert_eq!(answer, 1570930232582);
    }
}
