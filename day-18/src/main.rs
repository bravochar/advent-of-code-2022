const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::cmp::Ordering;
use std::{fmt, vec};
use std::fs::{read, File};
use std::io::{prelude::*, BufReader};

fn read_file(filename: &str) -> Vec<[i32; 3]> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line_str = line.unwrap();
        let line_str = line_str.trim();
        let coords: Vec<i32> = line_str.split(",")
            .map(|s| {s.parse().unwrap()})
            .collect();
        rval.push([coords.get(0).unwrap().to_owned(),
            coords.get(1).unwrap().to_owned(),
            coords.get(2).unwrap().to_owned()]);
        
    }
    rval
}

fn sort_by_x_y_z(a: &[i32; 3], b: &[i32; 3]) -> Ordering {
    let mut rval = a[0].cmp(&b[0]);
    if rval == Ordering::Equal {
        rval = a[1].cmp(&b[1]);
    }
    if rval == Ordering::Equal {
        rval = a[2].cmp(&b[2]);
    }
    rval
}

fn sort_by_y_z_x(a: &[i32; 3], b: &[i32; 3]) -> Ordering {
    let mut rval = a[1].cmp(&b[1]);
    if rval == Ordering::Equal {
        rval = a[2].cmp(&b[2]);
    }
    if rval == Ordering::Equal {
        rval = a[0].cmp(&b[0]);
    }
    rval
}

fn sort_by_z_x_y(a: &[i32; 3], b: &[i32; 3]) -> Ordering {
    let mut rval = a[2].cmp(&b[2]);
    if rval == Ordering::Equal {
        rval = a[0].cmp(&b[0]);
    }
    if rval == Ordering::Equal {
        rval = a[1].cmp(&b[1]);
    }
    rval
}

fn part_1(mut cubes: Vec<[i32; 3]>) -> i32 {
    let mut surface_area = 6 * cubes.len() as i32;

    cubes.sort_by(sort_by_x_y_z);
    for w in cubes.windows(2) {
        if w[0][0] == w[1][0]
                && w[0][1] == w[1][1]
                && w[1][2] - w[0][2] == 1 {
            surface_area -= 2;
        }
    }

    cubes.sort_by(sort_by_y_z_x);
    for w in cubes.windows(2) {
        if w[0][2] == w[1][2]
                && w[0][1] == w[1][1]
                && w[1][0] - w[0][0] == 1 {
            surface_area -= 2;
        }
    }

    cubes.sort_by(sort_by_z_x_y);
    for w in cubes.windows(2) {
        if w[0][0] == w[1][0]
                && w[0][2] == w[1][2]
                && w[1][1] - w[0][1] == 1 {
            surface_area -= 2;
        }
    }


    surface_area
}

fn part_2(mut cubes: Vec<[i32; 3]>) -> i32 {
    let mut surface_area = part_1(cubes.clone());

    cubes.sort_by(sort_by_x_y_z);
    for w in cubes.windows(2) {
        if w[0][0] == w[1][0]
                && w[0][1] == w[1][1] 
                && w[1][2] - w[0][2] > 1 {
            // TODO: search for the 4 enclosing points
            let x = w[0][0];
            let y = w[0][1];

            let mut total_enclosure = true;
            let z_bot = w[0][2] + 1;
            let z_top = w[1][2];
            let volume = z_top - z_bot;

            for z in z_bot..z_top {
                // need to find points with same Y, but +/-1 X
                if !cubes.iter().any(|c|{
                        c[0] == x + 1 && c[1] == y && c[2] == z})
                    || !cubes.iter().any(|c|{
                        c[0] == x - 1 && c[1] == y && c[2] == z})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y + 1 && c[2] == z})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y - 1 && c[2] == z}) {
                    total_enclosure = false;
                    break;
                }
            }
            if total_enclosure {
                println!("found enclosure of volume {}",
                    volume);
                surface_area -= 2 + 4 * volume;
            }

            // need to find points with same X, but +/-1 Y
        }
    }

    // XXX: start by finding single air cubes
    cubes.sort_by(sort_by_y_z_x);
    for w in cubes.windows(2) {
        if w[1][0] - w[0][0] > 2 
                && w[0][1] == w[1][1] 
                && w[1][2] == w[0][2] {
            // TODO: search for the 4 enclosing points
            let y = w[0][1];
            let z = w[0][2];

            let mut total_enclosure = true;
            let x_bot = w[0][0] + 1;
            let x_top = w[1][0];
            let volume = x_top - x_bot;

            for x in x_bot..x_top {
                // need to find points with same Y, but +/-1 X
                if !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y && c[2] == z + 1})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y && c[2] == z - 1})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y + 1 && c[2] == z})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y - 1 && c[2] == z}) {
                    total_enclosure = false;
                    break;
                }
            }
            if total_enclosure {
                println!("found enclosure of volume {}",
                    volume);
                surface_area -= 2 + 4 * volume;
            }

            // need to find points with same X, but +/-1 Y
        }
    }

    cubes.sort_by(sort_by_z_x_y);
    for w in cubes.windows(2) {
        if w[1][1] - w[0][1] > 2 
                && w[0][0] == w[1][0] 
                && w[1][2] == w[0][2] {
            // TODO: search for the 4 enclosing points
            let z = w[0][2];
            let x = w[0][0];

            let mut total_enclosure = true;
            let y_bot = w[0][1] + 1;
            let y_top = w[1][1];
            let volume = y_top - y_bot;

            for y in y_bot..y_top {
                // need to find points with same Y, but +/-1 X
                if !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y && c[2] == z + 1})
                    || !cubes.iter().any(|c|{
                        c[0] == x && c[1] == y && c[2] == z - 1})
                    || !cubes.iter().any(|c|{
                        c[0] == x + 1 && c[1] == y && c[2] == z})
                    || !cubes.iter().any(|c|{
                        c[0] == x - 1 && c[1] == y && c[2] == z}) {
                    total_enclosure = false;
                    break;
                }
            }
            if total_enclosure {
                println!("found enclosure of volume {}",
                    volume);
                surface_area -= 2 + 4 * volume;
            }

            // need to find points with same X, but +/-1 Y
        }
    }

    surface_area
}

fn main() {
    println!("Advent of Code, Day 18");

    // read in the input
    let cubes = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(cubes.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 64);
    }

    let now = Instant::now();
    let answer = part_2(cubes.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 58);
    }
}
