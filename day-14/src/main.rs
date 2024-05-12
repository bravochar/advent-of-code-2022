
use std::fmt;
use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum PointState {
    Air,
    Sand,
    Rock
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

const SAND_SOURCE: Point = Point{x: 500, y: 0};

struct Rock {
    line: Vec<Point>
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rval = fmt::Result::Ok(());

        if self.line.len() > 0 {
            let p = self.line.get(0).unwrap();
            rval = write!(f, "{},{}", p.x, p.y);
        }

        if rval.is_ok() && self.line.len() > 1 {
            for p in self.line.iter().skip(1) {
                rval = write!(f, " -> {},{}", p.x, p.y);
                if rval.is_err() {
                    return rval;
                }
            }
        }

        rval
    }
}

fn read_file() -> Vec<Rock> {
    let mut rval: Vec<Rock> = Vec::new();

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

        let mut rock = Rock{ line: Vec::new() };
        for point_str in line.trim().split(" -> ") {
            let mut coords = point_str.split(',');
            let x_str = coords.next().unwrap();
            let y_str = coords.next().unwrap();

            rock.line.push(Point{x: x_str.parse().unwrap(), y: y_str.parse().unwrap()})
        }

        rval.push(rock);
    }
    rval
}

fn part_1() {
    let rock_formation = read_file();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for rock in rock_formation.iter() {
        for p in rock.line.iter() {
            if p.x > max_x {
                max_x = p.x
            }
            if p.y > max_y {
                max_y = p.y
            }
        }
        println!("{}", rock);
    }

    max_x += 1;
    max_y += 1;
    println!("Working with grid of {} by {}", max_x, max_y);
    let mut grid: Vec<PointState> = Vec::with_capacity(max_x * max_y);
    for _ in 0..grid.capacity() {
        grid.push(PointState::Air);
    }

    // fill grid with rock
    for rock in rock_formation.iter() {
        let mut p_1 = rock.line.first().unwrap();
        
        for p_2 in rock.line.iter().skip(1) {
            for x in cmp::min(p_1.x, p_2.x)..(cmp::max(p_1.x, p_2.x) + 1) {
                for y in cmp::min(p_1.y, p_2.y)..(cmp::max(p_1.y, p_2.y) + 1) {
                    let p = grid.get_mut(x + max_x * y).unwrap();
                    *p = PointState::Rock;
                }
            }
            p_1 = p_2;
        }
    }

    let mut cur_x = SAND_SOURCE.x;
    let mut cur_y = SAND_SOURCE.y;
    let mut next_y = SAND_SOURCE.y;
    //let mut i = 0;
    let mut num_grains = 0;

    loop {
        // get next point
        loop {
            let p = grid.get(
                cur_x + max_x * next_y).unwrap();

            if PointState::Air.eq(p) {
                cur_y = next_y;
                next_y += 1;
                break;
            }

            if cur_x > 1 {
                let p = grid.get(
                    cur_x - 1 + max_x * next_y).unwrap();

                if PointState::Air.eq(p) {
                    cur_x -= 1;
                    cur_y = next_y;
                    next_y += 1;
                    break;
                }
            }

            if cur_x < (max_x - 2) {
                let p = grid.get(
                    cur_x + 1 + max_x * next_y).unwrap();

                if PointState::Air.eq(p) {
                    cur_x += 1;
                    cur_y = next_y;
                    next_y += 1;
                    break;
                }
            }
            
            let p = grid.get_mut(
                cur_x + max_x * cur_y).unwrap();
            *p = PointState::Sand;
            //println!("Changed state of point {},{} to SAND", cur_x, cur_y);
            cur_x = SAND_SOURCE.x;
            cur_y = SAND_SOURCE.y;
            next_y = SAND_SOURCE.y;
            num_grains += 1;
        }

        if next_y >= max_y {
            println!("Sand fell thorugh after {} grains", num_grains);
            break;

        } else {
            //println!("Sand moved to {},{}", cur_x, cur_y);
        }
    }

}

fn part_2() {
    let rock_formation = read_file();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for rock in rock_formation.iter() {
        for p in rock.line.iter() {
            if p.x > max_x {
                max_x = p.x
            }
            if p.y > max_y {
                max_y = p.y
            }
        }
        println!("{}", rock);
    }

    max_x *= 2;
    max_y += 3;
    println!("Working with grid of {} by {} for part 2", max_x, max_y);
    let mut grid: Vec<PointState> = Vec::with_capacity(max_x * max_y);
    for _ in 0..grid.capacity() {
        grid.push(PointState::Air);
    }

    // fill grid with rock
    for rock in rock_formation.iter() {
        let mut p_1 = rock.line.first().unwrap();
        
        for p_2 in rock.line.iter().skip(1) {
            for x in cmp::min(p_1.x, p_2.x)..(cmp::max(p_1.x, p_2.x) + 1) {
                for y in cmp::min(p_1.y, p_2.y)..(cmp::max(p_1.y, p_2.y) + 1) {
                    let p = grid.get_mut(x + max_x * y).unwrap();
                    *p = PointState::Rock;
                }
            }
            p_1 = p_2;
        }
    }

    let mut cur_x = SAND_SOURCE.x;
    let mut cur_y = SAND_SOURCE.y;
    let mut next_y = SAND_SOURCE.y;
    //let mut i = 0;
    let mut num_grains = 0;

    loop {
        // get next point
        loop {
            let p = grid.get(
                cur_x + max_x * next_y).unwrap();

            if PointState::Air.eq(p) {
                cur_y = next_y;
                next_y += 1;
                break;
            }

            if cur_x > 1 {
                let p = grid.get(
                    cur_x - 1 + max_x * next_y).unwrap();

                if PointState::Air.eq(p) {
                    cur_x -= 1;
                    cur_y = next_y;
                    next_y += 1;
                    break;
                }
            }

            if cur_x < (max_x - 2) {
                let p = grid.get(
                    cur_x + 1 + max_x * next_y).unwrap();

                if PointState::Air.eq(p) {
                    cur_x += 1;
                    cur_y = next_y;
                    next_y += 1;
                    break;
                }
            }

            next_y = cur_y;
            break;
        }

        if (next_y == cur_y) || (next_y == max_y - 1) {
            let p = grid.get_mut(
                cur_x + max_x * cur_y).unwrap();
            *p = PointState::Sand;

            //println!("Changed state of point {},{} to SAND", cur_x, cur_y);
            cur_x = SAND_SOURCE.x;
            cur_y = SAND_SOURCE.y;
            next_y = SAND_SOURCE.y;
            num_grains += 1;

            continue

        } else {
            //println!("Sand moved to {},{}", cur_x, cur_y);
        }
        let p = grid.get_mut(
            SAND_SOURCE.x + max_x * SAND_SOURCE.y).unwrap();
        if PointState::Sand.eq(p) {
            println!("Sand stops after {} grains", num_grains);
            break
        }

    }

}

fn main() {
    println!("Advent of Code, Day 14");

    part_1();
    part_2();
}