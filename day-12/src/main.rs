
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

#[derive(Debug)]
#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
#[derive(Clone)]
struct Hill {
    h_char: char,
    height: u32,
    visited: bool
}

impl Hill {
    fn from_height(h_char: char) -> Self {
        let height: u32 = match h_char {
            'S' => 0,
            'E' => 26,
            'a'..='z' => h_char as u32 - 'a' as u32,
            _ => panic!("Invalid height charcter {:?}", h_char),

        };

        Self {
            h_char,
            height,
            visited: false,
        }
    }
}

fn print_hills(hills: &Vec::<Vec::<Hill>>) {
    for row in hills {
        for h in row {
            print!("{}", h.h_char);
        }
        println!();
    }
}

// read in the file and store in a 2-D vector
fn read_hill_grid() -> Vec::<Vec::<Hill>> {
    let mut rows = Vec::<Vec::<Hill>>::new();

    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let line_str = line_str.trim();

        let mut row = Vec::<Hill>::new();
        for c in line_str.chars() {
            row.push(Hill::from_height(c));
        }
        rows.push(row);
    }

    print_hills(&rows);

    rows
}


fn mark_visited(p: &Point, hills: &mut[Vec<Hill>]) {
    let row = hills.get_mut(p.y).unwrap();
    let h : &mut Hill = row.get_mut(p.x).unwrap();

    h.visited = true
}

fn check_if_visited(p: &Point, hills: &[Vec<Hill>]) -> bool {
    hills.get(p.y).unwrap().get(p.x).unwrap().visited
}

fn get_next_steps(pos: &Point, hills: &mut[Vec<Hill>]) -> Vec::<Point> {
    let max_y = hills.len() - 1;
    let first_row = hills.first().unwrap();
    let max_x = first_row.len() - 1;

    // start with those possible from the contraints of the grid
    let mut steps = Vec::<Point>::new();
    if pos.x > 0 {
        steps.push( Point {x: pos.x - 1, y: pos.y});
    }

    if pos.x < max_x {
        steps.push( Point {x: pos.x + 1, y: pos.y});
    }
    if pos.y > 0 {
        steps.push( Point {y: pos.y - 1, x: pos.x});
    }

    if pos.y < max_y {
        steps.push( Point {y: pos.y + 1, x: pos.x});
    }

    // check step for valid elevation change
    let mut rval = Vec::<Point>::new();
    let cur_h = hills.get(pos.y).unwrap().get(pos.x).unwrap();
    for step in steps {
        let step_row = hills.get(step.y).unwrap();
        let step_h = step_row.get(step.x).unwrap();

        // check if visited
        if check_if_visited(&step, hills) {
            continue;

        } else {
            //println!("  Already visited {:?}", step);
        }

        // check height
        if step_h.height <= (cur_h.height + 1) {
            rval.push(step);
        } else {
            //println!("  Bad move {:?} to {:?}", cur_h, step_h);
        }
    }

    rval
}

fn take_step_on_path(path: Vec::<Point>, hills: &mut[Vec<Hill>]) -> Vec::<Vec::<Point>> {
    let mut ret_paths = Vec::<Vec::<Point>>::new();

    // get list of points we can move to
    let steps = get_next_steps(path.last().unwrap(), hills);

    // check step for valid elevation change
    for step in steps {
        // mark step as visited
        mark_visited(&step, hills);

        // add new path to ret_paths
        let mut new_path = path.clone();
        new_path.push(step);
        ret_paths.push(new_path);
    }

    ret_paths
}

fn take_a_step(paths: Vec::<Vec::<Point>>, mut hills: Vec::<Vec::<Hill>>) -> (Vec::<Vec::<Point>>, Vec::<Vec::<Hill>>) {
    let mut ret_paths = Vec::<Vec::<Point>>::new();

    for path in paths {
        let new_paths = take_step_on_path(path, &mut hills);

        for new_path in new_paths {
            ret_paths.push(new_path);
        }
    }

    (ret_paths, hills)
}

fn find_best_path(mut paths: Vec::<Vec::<Point>>, mut hills: Vec::<Vec::<Hill>>, e: Point) -> Vec<Point> {
    // mark all starting points as visited
    for path in &paths {
        mark_visited(path.last().unwrap(), &mut hills);
    }

    // main loop
    //print_paths(&paths);
    while !paths.is_empty() {
        // take another step along each path
        (paths, hills) = take_a_step(paths, hills);
        //print_paths(&paths);

        // check if we're at the end
        for path in &paths {
            let p = path.last().unwrap();

            // return shortest path
            if p.x == e.x && p.y == e.y {
                return path.clone();
            }
        }
    }

    println!("Could not find path to end {:?}", e);
    panic!("Could not find path to end");
}

fn find_best_path_from_point(mut hills: Vec::<Vec::<Hill>>, s: Point, e: Point) -> Vec<Point> {
    mark_visited(&s, &mut hills);

    // declare a vector of vectors of points to hold all paths we're exploring
    let starting_path = vec![s.clone()];
    let paths = vec![starting_path];

    find_best_path(paths, hills, e)
}

/*
fn print_paths(paths: &Vec::<Vec::<Point>>) {
    println!("Paths:");
    for path in paths {
        println!("  {:?}", path);
    }
}
*/

fn find_hill_char(hills: &[Vec<Hill>], c: char) -> Result<Point, String> {
    for (y, row) in hills.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if h.h_char == c {
                return Ok(Point {x, y});
            }
        }
    }

    Err(format!("Could not find char '{}'", c))
}

fn part_1() {

    let hills = read_hill_grid();

    // find start
    let start = find_hill_char(&hills, 'S').unwrap();
    println!("Found start at {:?}", start);

    // find end
    let end = find_hill_char(&hills, 'E').unwrap();
    println!("Found end at {:?}", end);

    let max_y = hills.len() - 1;
    let first_row = hills.first().unwrap();
    let max_x = first_row.len() - 1;
    println!("Total points are {}", max_x * max_y);

    // XXX: At this point, I went on a tangent where I tried to define a struct
    // that was essentially a linked list and the compiler kept getting _very_
    // upset with me as a person.

    // Instead, we should just create a vector of points to represent a path.
    // We should expand the path in a breadth-first manner, and the first path
    // to complete is the shortest. Whenever a path has no more valid moves, it
    // can be destroyed. A 2-D vector of visited points should be maintained,
    // as any visited point is not a valid move - some other path got to that
    // point faster.
    let path = find_best_path_from_point(hills.clone(), start, end);

    println!("Best path takes {} steps", path.len() - 1);
}

fn part_2() {
    let hills = read_hill_grid();

    // find start
    let start = find_hill_char(&hills, 'S').unwrap();
    println!("Found start at {:?}", start);

    // find end
    let end = find_hill_char(&hills, 'E').unwrap();
    println!("Found end at {:?}", end);

    let max_y = hills.len() - 1;
    let first_row = hills.first().unwrap();
    let max_x = first_row.len() - 1;
    println!("Total points are {}", max_x * max_y);

    // declare a vector of vectors of points to hold all paths we're exploring
    let starting_path = vec![start.clone()];
    let mut paths = vec![starting_path];

    // find all points with 'a' and add to starting paths
    for (y, row) in hills.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if h.h_char == 'a' {
                let path = vec![Point {x, y}];
                paths.push(path);
            }
        }
    }
    println!("Found {} starting points", paths.len());

    let path = find_best_path(paths, hills.clone(), end);

    println!("Best path takes {} steps", path.len() - 1);
}


fn main() {
    println!("Advent of Code, Day 12");

    part_1();
    part_2();
}


