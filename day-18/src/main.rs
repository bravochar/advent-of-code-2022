const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::cmp::Ordering;
use std::fs::File;
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

fn get_next_points(p: &[i32; 3], max: [i32; 3]) -> Vec<[i32; 3]> {
    let mut rval = Vec::new();

    let x = p[0];
    let y = p[1];
    let z = p[2];

    if x > -1 {
        rval.push([x - 1, y, z]);
    }
    if y > -1 {
        rval.push([x, y - 1, z]);
    }
    if z > -1 {
        rval.push([x, y, z - 1]);
    }
    if x <= max[0] {
        rval.push([x + 1, y, z]);
    }
    if y <= max[1] {
        rval.push([x, y + 1, z]);
    }
    if z <= max[1] {
        rval.push([x, y, z + 1]);
    }

    rval
}

fn find_total_surface_area(mut cubes: Vec<[i32; 3]>) -> i32 {
    let mut surface_area = 0;

    cubes.sort();

    /*
     * NEW APPROACH: Explore the whole grid by moving into all contiguous
     * unoccupied blocks and counting all neighboring blocks (1 unit added
     * per neighbor to surface area). Keep track of visited spaces to prevent
     * repetition. The loop should be:
     *
     *  1) explore all 6 direct neighbors (no diagonals)
     *      *) previously visited unoccupied squares are ignore
     *      *) unexplored unoccupied squares are added to the queue and marked
     *          as visited (more accurately to-be-visited
     *      *) occupied spaces are counted as 1 unit of surface area
     *
     * We'll need to start at -1,-1,-1 to count the zero-facing surfaces, and
     * continue to max(coord) + 1 in each axis
     */
    let max_x = cubes.iter()
        .map(|c| { c[0] })
        .max().unwrap();
    let max_y = cubes.iter()
        .map(|c| { c[1] })
        .max().unwrap();
    let max_z = cubes.iter()
        .map(|c| { c[2] })
        .max().unwrap();
    let max_point = [max_x + 3, max_y + 3, max_z + 3];

    let mut queue = Vec::new();
    queue.push([-1,-1, -1]);
    let mut visited = Vec::new();
    visited.push([-1,-1, -1]);

    /*
    println!("Exploring from {:?} to {:?}",
        [-1,-1, -1],
        max_point);
    */

    while !queue.is_empty() {
        let p = queue.pop().unwrap();
        //println!("{:?}", p);

        for n in get_next_points(&p, max_point) {
            match cubes.binary_search(&n) {
                Ok(_) => {
                    /*
                    println!("Found {:?} (from {:?})",
                        n, p);
                     */
                    surface_area += 1;
                    continue;
                },
                _ => ()
            };
            match visited.binary_search(&n) {
                Ok(_) => {
                    continue;
                },
                Err(i) => {
                    visited.insert(i, n);
                    queue.push(n);
                }
            };
        }
    }

    surface_area
}

fn part_2(cubes: Vec<[i32; 3]>) -> i32 {

    find_total_surface_area(cubes)
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
