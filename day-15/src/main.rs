
use std::cmp::min;
use std::fmt;
use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

//const FILENAME: &str = "./input";
const FILENAME: &str = "./test";

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Sensor {
    loc: Point,
    beacon: Point,
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
    fn man_dist(&self, p: &Point) -> i32 {
        (self.loc.x - p.x).abs() + (self.loc.y - p.y).abs()
    }

    fn excluded_point(&self, p: &Point) -> bool {
        self.man_dist(p) <= self.sensor_man_dist()
    }

    fn sensor_man_dist(&self) -> i32 {
        self.man_dist(&self.beacon)
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

        rval.push(Sensor {
            loc: Point {
                x: s_x_str.parse().unwrap(),
                y: s_y_str.parse().unwrap()},
            beacon: Point {
                x: b_x_str.parse().unwrap(),
                y: b_y_str.parse().unwrap()}
        });
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
        println!("{}", sensor);
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
                    println!("Point {} excluded by sensor at {} ({} <= {})",
                        p,
                        s.loc,
                        s.man_dist(&p),
                        s.sensor_man_dist());
                    is_excluded = true;

                    // exclude all points close to this sensor
                    while is_excluded {
                        excluded_points += 1;
                        i += 1;
                        let p = Point{x: i, y: target_row};
                        is_excluded = s.excluded_point(&p);

                        if !is_excluded {
                            println!("Point {} free of sensor at {} ({} > {})",
                                p,
                                s.loc,
                                s.man_dist(&p),
                                s.sensor_man_dist());
                        }
                    }
                    i -= 1;

                    // final check, is there a beacon here?
                    if s.beacon.y == target_row  && s.beacon.x == i {
                        println!("Beacon at {}, decrementeing excluded points",
                            s.beacon);
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
}

fn main() {
    println!("Advent of Code, Day 15");

    part_1();
    part_2();
}