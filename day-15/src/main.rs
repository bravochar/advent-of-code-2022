
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Sensor {
    loc: Point,
    beacon: Point,
    beacon_dist: i32
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            self.loc.x,
            self.loc.y,
            self.beacon.x,
            self.beacon.y)
    }
}

impl Sensor {
    fn new(l_x: i32, l_y: i32, b_x: i32, b_y: i32) -> Sensor {
        let loc = Point::new(l_x, l_y);
        let beacon = Point::new(b_x, b_y);
        let beacon_dist = (loc.x - beacon.x).abs() + (loc.y - beacon.y).abs();

        Sensor{loc, beacon, beacon_dist}
    }

    fn next_free_y(&self, p: &Point) -> i32 {
        let x_dist = (self.loc.x - p.x).abs();
        if x_dist < self.beacon_dist {
            let y_dist = (self.loc.y - p.y).abs();
            let rem_dist = self.beacon_dist - x_dist;

            if y_dist <= rem_dist {
                let new_y = self.loc.y + rem_dist + 1;

                if new_y > p.y {
                    return new_y;
                }
            }
        }
        return p.y
    }

    fn man_dist(&self, p: &Point) -> i32 {
        (self.loc.x - p.x).abs() + (self.loc.y - p.y).abs()
    }

    fn excluded_point(&self, p: &Point) -> bool {
        self.man_dist(p) <= self.beacon_dist
    }
}

fn read_file() -> Vec<Sensor> {
    let mut rval: Vec<Sensor> = Vec::new();

    // Open the file
    let file = File::open(FILENAME).unwrap();
    let mut reader = BufReader::new(file);
    // Read file line by line
    loop {
        let mut line = Default::default();
        let line_size = reader.read_line(&mut line).unwrap();
        match line_size  {
            0 => break, // EOF
            _ => (),
        };

        // pull out all of the coords
        let (_, line) = line.trim().split_once("=").unwrap();
        let (s_x_str, line) = line.split_once(",").unwrap();
        let (_, line) = line.trim().split_once("=").unwrap();
        let (s_y_str, line) = line.split_once(":").unwrap();
        let (_, line) = line.trim().split_once("=").unwrap();
        let (b_x_str, line) = line.split_once(",").unwrap();
        let (_, b_y_str) = line.trim().split_once("=").unwrap();

        rval.push(Sensor::new(
            s_x_str.parse().unwrap(),
            s_y_str.parse().unwrap(),
            b_x_str.parse().unwrap(),
            b_y_str.parse().unwrap()));
    }

    rval
}

fn part_1() {
    let sensors = read_file();

    let mut max_x: i32 = 0;
    let mut min_x: i32 = 0;
    for sensor in sensors.iter() {
        if sensor.loc.x + sensor.man_dist(&sensor.beacon) > max_x {
            max_x = sensor.loc.x + sensor.man_dist(&sensor.beacon);
        }
        if sensor.loc.x - sensor.man_dist(&sensor.beacon) < min_x {
            min_x = sensor.loc.x - sensor.man_dist(&sensor.beacon);
        }
    }

    let mut target_row = 2000000;
    if FILENAME.eq("./test") {
        target_row = 10;
    }
    let mut excluded_points = 0;
    println!("Checking for sensors {}-{}", min_x, max_x);
    let mut i = min_x;
    while i < max_x {
        let p = Point{x: i, y: target_row};
        let mut is_beacon = false;
        for s in sensors.iter() {
            if s.beacon.x == i && s.beacon.y == target_row {
                is_beacon = true;
                break;
            }
        }

        if !is_beacon {
            let mut is_excluded = false;
            for s in sensors.iter() {
                if !is_excluded && s.excluded_point(&p) {
                    is_excluded = true;

                    // exclude all points close to this sensor
                    while is_excluded {
                        excluded_points += 1;
                        i += 1;
                        let p = Point{x: i, y: target_row};
                        is_excluded = s.excluded_point(&p);
                    }
                    i -= 1;

                    // final check, is there a beacon here?
                    if s.beacon.y == target_row  && s.beacon.x == i {
                        excluded_points -= 1;
                    }
                    break;
                }
            }
        }

        i += 1;
    }

    println!("Excluded points: {}", excluded_points);

}

fn part_2() {
    let sensors = read_file();

    let mut max_x: i32 = 0;
    let mut min_x: i32 = 0;
    for sensor in sensors.iter() {
        if sensor.loc.x + sensor.man_dist(&sensor.beacon) > max_x {
            max_x = sensor.loc.x + sensor.man_dist(&sensor.beacon);
        }
        if sensor.loc.x - sensor.man_dist(&sensor.beacon) < min_x {
            min_x = sensor.loc.x - sensor.man_dist(&sensor.beacon);
        }
    }

    let mut max_coord = 4000000;
    if FILENAME.eq("./test") {
        max_coord = 20;
    }
    println!("Checking for sensors {}-{}", 0, max_coord);
    for x in 0..max_coord {
        let mut y = 0;
        while y < max_coord {
            let mut completely_free = true;
            
            for s in sensors.iter() {
                let p = Point{x, y};
                let new_y = s.next_free_y(&p);

                if new_y > y {
                    completely_free = false;
                    y = new_y;
    
                    if y > max_coord {
                        break;
                    }
                }
            }

            if completely_free {
                println!("Point {},{} was not excluded", x, y);
                let x: i64 = x.into();
                let y: i64 = y.into();
                let freq: i64 = 4000000 * x + y;
                println!("Tuning frequency is {}",
                    freq);
                return
            }
        }
    }

}

fn main() {
    println!("Advent of Code, Day 15");

    part_1();
    part_2();
}