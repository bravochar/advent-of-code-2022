use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}

impl Tree {
    fn from_height(height: u32) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

// read in the file and store in a 2-D vector
fn read_tree_grid() -> Vec::<Vec::<Tree>> {
    let mut rows = Vec::<Vec::<Tree>>::new();

    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let line_str = line_str.trim();
        println!("{}", line_str);

        let mut row = Vec::<Tree>::new();
        for c in line_str.chars() {
            let h = c.to_digit(10).expect("Error: height was not number.");

            row.push(Tree::from_height(h));
        }
        rows.push(row);
    }

    rows
}

fn mark_visible_trees(mut trees: Vec::<Vec::<Tree>>) -> Vec::<Vec::<Tree>> {
    let mut sum = 0;
    let num_rows = trees.len();
    let row_len = trees[0].len();

    // walk the top side
    for y in 0..row_len {
        let mut h = 0;
        for (x, row) in  trees.iter_mut().enumerate() {
            let t: &mut Tree = &mut row[y];

            // tree is visible if it is taller than previously seen tallest tree
            if t.height >= h {
                if !t.visible {
                    println!("Tree[{}][{}]({}) visible; (prev: {})", x, y, t.height, h);
                    t.visible = true;
                    sum += 1;
                }

                if t.height < 9 {
                    h = t.height + 1
                } else {
                    // once we hit the max height, no more trees are visible
                    break;
                }
            }
        }
    }

    // walk the right side
    for (x, row) in  trees.iter_mut().enumerate() {
        let mut h = 0;
        for y in (0..row_len).rev() {
            let t: &mut Tree = &mut row[y];

            // tree is visible if it is taller than previously seen tallest tree
            if t.height >= h {
                if !t.visible {
                    println!("Tree[{}][{}]({}) visible; (prev: {})", x, y, t.height, h);
                    t.visible = true;
                    sum += 1;
                }

                if t.height < 9 {
                    h = t.height + 1
                } else {
                    // once we hit the max height, no more trees are visible
                    break;
                }
            }
        }
    }

    // walk the bottom side
    for y in (0..row_len).rev() {
        let mut h = 0;
        for x in (0..num_rows).rev() {
            let t: &mut Tree = &mut trees[x][y];

            // tree is visible if it is taller than previously seen tallest tree
            if t.height >= h {
                if !t.visible {
                    println!("Tree[{}][{}]({}) visible; (prev: {})", x, y, t.height, h);
                    t.visible = true;
                    sum += 1;
                }

                if t.height < 9 {
                    h = t.height + 1
                } else {
                    // once we hit the max height, no more trees are visible
                    break;
                }
            }
        }
    }

    println!("walk the left side");
    for x in (0..num_rows).rev() {
        let mut h = 0;
        for y in 0..row_len {
            let t: &mut Tree = &mut trees[x][y];

            // tree is visible if it is taller than previously seen tallest tree
            if t.height >= h {
                if !t.visible {
                    println!("Tree[{}][{}]({}) visible; (prev: {})", x, y, t.height, h);
                    t.visible = true;
                    sum += 1;
                }

                if t.height < 9 {
                    h = t.height + 1
                } else {
                    // once we hit the max height, no more trees are visible
                    break;
                }
            }
        }
    }
    println!("Marked {} trees as visible", sum);

    trees
}

fn sum_visible_trees(trees: Vec::<Vec::<Tree>>) -> u32 {
    let mut sum = 0;

    let trees = mark_visible_trees(trees);

    for row in trees.iter() {
        for tree in row.iter() {
            if tree.visible {
                sum += 1;
            }
        }
    }

    for row in trees.iter() {
        println!("{:?}", row);
    }

    sum
}

fn part_1() -> u32 {

    let trees = read_tree_grid();

    // Print the answer to the first part
    let answer = sum_visible_trees(trees);
    println!("First Answer: {:?}", answer);

    answer
}

fn calc_vis_score(trees: &[Vec::<Tree>], x: usize, y: usize) -> usize {
    let mut score = 1;
    let t = &trees[x][y];

    // look up
    let mut s = 0;
    for row in trees.iter().take(x).rev() {
        s += 1;
        if row[y].height >= t.height {
            break;
        }
    }
    score *= s;

    // look right
    let mut s = 0;
    for t2 in trees[x].iter().skip(y+1) {
        s += 1;
        if t2.height >= t.height {
            break;
        }
    }
    score *= s;

    // look down
    let mut s = 0;
    for row in trees.iter().skip(x+1) {
        s += 1;
        if row[y].height >= t.height {
            break;
        }
    }
    score *= s;

    // look right
    let mut s = 0;
    for t2 in trees[x].iter().take(y).rev() {
        s += 1;
        if t2.height >= t.height {
            break;
        }
    }
    score *= s;

    score
}

fn find_max_vis_score(trees: &[Vec::<Tree>]) -> usize {
    let mut score = 0;
    let num_rows = trees.len();
    let row_len = trees[0].len();

    for x in 1..num_rows-1 {
        for y in 1..row_len-1 {
            let s = calc_vis_score(trees, x, y);
            if s > score {
                score = s;
            }
        }
    }

    score
}

fn part_2() -> usize {

    let trees = read_tree_grid();

    // Print the answer to the second part
    let answer = find_max_vis_score(&trees);
    println!("Second Answer: {:?}", answer);

    answer
}

fn main() {
    println!("Advent of Code, Day 8");

    assert_eq!(part_1(), 1789);
    assert_eq!(part_2(), 314820);
}

