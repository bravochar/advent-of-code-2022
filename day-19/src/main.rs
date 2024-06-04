const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::cmp::Ordering;
use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::collections::BinaryHeap;

#[derive(Clone,Debug)]
struct Blueprint {
    id: i32,
    ore_robot: [i32; 3],
    clay_robot: [i32; 3],
    obsidian_robot: [i32; 3],
    geode_robot: [i32; 3],
    best_score: i32
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blueprint {}: \
            Each ore robot costs {} ore. \
            Each clay robot costs {} ore. \
            Each obsidian robot costs {} ore and {} clay. \
            Each geode robot costs {} ore and {} obsidian.",
            self.id,
            self.ore_robot[0],
            self.clay_robot[0],
            self.obsidian_robot[0],
            self.obsidian_robot[1],
            self.geode_robot[0],
            self.geode_robot[2])
    }
}

impl Blueprint {

    fn from_line(line: &str) -> Blueprint {
        let (_, line) = line.split_once(" ").unwrap();
        let (id, line) = line.split_once(":").unwrap();
        let id = id.parse().unwrap();

        let (_, line) = line.split_once("costs ").unwrap();
        let (ore, line) = line.split_once(" ore").unwrap();
        let ore_robot = [ore.parse().unwrap(), 0, 0];

        let (_, line) = line.split_once("costs ").unwrap();
        let (ore, line) = line.split_once(" ore").unwrap();
        let clay_robot = [ore.parse().unwrap(), 0, 0];

        let (_, line) = line.split_once("costs ").unwrap();
        let (ore, line) = line.split_once(" ore and ").unwrap();
        let (clay, line) = line.split_once(" clay").unwrap();
        let obsidian_robot = [ore.parse().unwrap(), clay.parse().unwrap(), 0];

        let (_, line) = line.split_once("costs ").unwrap();
        let (ore, line) = line.split_once(" ore and ").unwrap();
        let (obsidian, _) = line.split_once(" obsidian").unwrap();
        let geode_robot = [ore.parse().unwrap(), 0, obsidian.parse().unwrap()];

        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            best_score: 0
        }
    }
}

#[derive(Debug,Clone)]
#[derive(PartialEq, Eq)]
enum BuildRobot {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot
}

#[derive(Debug,Clone)]
struct Strategy<'v> {
    blueprint: &'v Blueprint,
    turn_num: i32,
    time_limit: i32,
    actions: HashMap<i32, BuildRobot>,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32
}
impl <'v> fmt::Display for Strategy<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Minute {}:
    {} geode, {} obsidian, {} clay, {} ore
    {} geode robots, {} obsidian robots, {} clay robots, {} ore_robots",
            self.turn_num,
            self.geodes,
            self.obsidian,
            self.clay,
            self.ore,
            self.geode_robots,
            self.obsidian_robots,
            self.clay_robots,
            self.ore_robots
            )
    }
}

impl <'v> Ord for Strategy<'v> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut rval = self.geodes.cmp(&other.geodes);
        if rval != Ordering::Equal {
            return rval;
        }

        rval = self.geode_robots.cmp(&other.geode_robots);
        if rval != Ordering::Equal {
            return rval;
        }
        rval = self.obsidian_robots.cmp(&other.obsidian_robots);
        if rval != Ordering::Equal {
            return rval;
        }
        rval = self.clay_robots.cmp(&other.clay_robots);
        if rval != Ordering::Equal {
            return rval;
        }
        rval = self.ore_robots.cmp(&other.ore_robots);
        if rval != Ordering::Equal {
            return rval;
        }

        rval = self.actions.len().cmp(&other.actions.len());
        if rval != Ordering::Equal {
            return rval;
        }

        rval = self.obsidian.cmp(&other.obsidian);
        if rval != Ordering::Equal {
            return rval;
        }
        rval = self.clay.cmp(&other.clay);
        if rval != Ordering::Equal {
            return rval;
        }
        rval = self.ore.cmp(&other.ore);
        if rval != Ordering::Equal {
            return rval;
        }

        rval = other.turn_num.cmp(&self.turn_num);
        if rval != Ordering::Equal {
            return rval;
        }

        rval
    }
}

impl <'v> Eq for Strategy<'v> {
}

impl <'v> PartialEq for Strategy<'v> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl <'v> PartialOrd for Strategy<'v> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <'v> Strategy<'v> {
    fn new(blueprint: &'v Blueprint, time_limit: i32) -> Strategy<'v> {
        // only thing that can happen first turn is that we produce
        // ore, so skip that step
        Strategy {
            turn_num: 1,
            time_limit: time_limit,
            blueprint,
            actions: HashMap::new(),
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0
        }
    }

    #[allow(dead_code)]
    fn print_strat(&self) {
        let mut ore = 0;
        let mut clay = 0;
        let mut obsidian = 0;
        let mut geodes = 0;
        let mut ore_robots = 1;
        let mut clay_robots = 0;
        let mut obsidian_robots = 0;
        let mut geode_robots = 0;
        let bp = self.blueprint;

        for minute in 1..self.turn_num {
            println!("== Minute {} ==", minute);
            // TODO: build robots part
            match self.actions.get(&minute) {
                Some(BuildRobot::OreRobot) => {
                    println!("Spend {} ore to start building an ore-collecting robot.",
                        bp.ore_robot[0]);
                    ore -= bp.ore_robot[0];
                },
                Some(BuildRobot::ClayRobot) => {
                    println!("Spend {} ore to start building an clay-collecting robot.",
                        bp.clay_robot[0]);
                    ore -= bp.clay_robot[0];
                },
                Some(BuildRobot::ObsidianRobot) => {
                    println!("Spend {} ore and {} clay to start building an obsidian-collecting robot.",
                        bp.obsidian_robot[0],
                        bp.obsidian_robot[1]);
                    ore -= bp.obsidian_robot[0];
                    clay -= bp.obsidian_robot[1];
                },
                Some(BuildRobot::GeodeRobot) => {
                    println!("Spend {} ore and {} obsidian to start building an geode-cracking robot.",
                        bp.geode_robot[0],
                        bp.geode_robot[2]);
                    ore -= bp.geode_robot[0];
                    obsidian -= bp.geode_robot[2];
                },
                _ => ()
            }

            // minerals
            if ore_robots > 0 {
                ore += ore_robots;
                println!("{} ore-collecting robot collect {} ore; you now have {} ore",
                    ore_robots, ore_robots, ore);
            }
            if clay_robots > 0 {
                clay += clay_robots;
                println!("{} clay-collecting robot collect {} or; you now have {} clay",
                    clay_robots, clay_robots, clay);
            }
            if obsidian_robots > 0 {
                obsidian += obsidian_robots;
                println!("{} obsidian-collecting robot collect {} or; you now have {} obsidian",
                    obsidian_robots, obsidian_robots, obsidian);
            }
            if geode_robots > 0 {
                geodes += geode_robots;
                println!("{} geode-collecting robot collect {} or; you now have {} geode",
                    geode_robots, geode_robots, geodes);
            }

            match self.actions.get(&(minute)) {
                Some(BuildRobot::OreRobot) => {
                    ore_robots += 1;
                    println!("The new ore-collection robot is ready; you now have {} of them.",
                        ore_robots);
                },
                Some(BuildRobot::ClayRobot) => {
                    clay_robots += 1;
                    println!("The new clay-collection robot is ready; you now have {} of them.",
                        clay_robots);
                },
                Some(BuildRobot::ObsidianRobot) => {
                    obsidian_robots += 1;
                    println!("The new obsidian-collection robot is ready; you now have {} of them.",
                        obsidian_robots);
                },
                Some(BuildRobot::GeodeRobot) => {
                    geode_robots += 1;
                    println!("The new geode-cracking robot is ready; you now have {} of them.",
                        geode_robots);
                },
                _ => ()
            }
            println!("");
        }

    }

    fn turns_for_robot(&self, minerals: &[i32; 3]) -> i32 {
        let mut rval = 0;
        let ore = minerals[0];
        let clay = minerals[1];
        let obsidian = minerals[2];

        // no use building a robot if it isn't going to produce resources,
        // means we need 2 extra turns - 1 to build it and 1 for it to produce
        let max_turns = self.time_limit - self.turn_num - 1;
        
        if self.obsidian < obsidian {
            if self.obsidian_robots < 1 {
                return -1;
            }
            let needed_obsidian = obsidian - self.obsidian;
            let turns = if needed_obsidian % self.obsidian_robots == 0 {
                needed_obsidian / self.obsidian_robots
            } else {
                needed_obsidian / self.obsidian_robots + 1
            };
            rval = max(rval, turns);
        }
        
        if self.clay < clay && rval <= max_turns {
            if self.clay_robots < 1 {
                return -1;
            }
            let needed_clay = clay - self.clay;
            let turns = if needed_clay % self.clay_robots == 0 {
                needed_clay / self.clay_robots
            } else {
                needed_clay / self.clay_robots + 1
            };
            rval = max(rval, turns);
        }
        
        if self.ore < ore && rval <= max_turns {
            let needed_ore = ore - self.ore;
            let turns = if needed_ore % self.ore_robots == 0 {
                needed_ore / self.ore_robots
            } else {
                needed_ore / self.ore_robots + 1
            };
            rval = max(rval, turns);
        }

        if rval > max_turns {
            return -1;
        }
        rval
    }

    fn wait_turns(&mut self, num_turns: i32) {
        for _ in 0..num_turns {
            self.ore += self.ore_robots;
            self.clay += self.clay_robots;
            self.obsidian += self.obsidian_robots;
            self.geodes += self.geode_robots;
            match self.actions.get(&self.turn_num) {
                Some(BuildRobot::GeodeRobot) => self.geode_robots += 1,
                Some(BuildRobot::ObsidianRobot) => self.obsidian_robots += 1,
                Some(BuildRobot::ClayRobot) => self.clay_robots += 1,
                Some(BuildRobot::OreRobot) => self.ore_robots += 1,
                _ => ()
            };
            self.turn_num += 1;
        }
    }

    fn build_geode_robot(&mut self) {
        self.ore -= self.blueprint.geode_robot[0];
        self.obsidian -= self.blueprint.geode_robot[2];

        assert!(self.ore >= 0);
        assert!(self.obsidian >= 0);

        self.actions.insert(self.turn_num, BuildRobot::GeodeRobot);
        self.wait_turns(1);
    }

    fn build_obsidian_robot(&mut self) {
        self.ore -= self.blueprint.obsidian_robot[0];
        self.clay -= self.blueprint.obsidian_robot[1];

        assert!(self.ore >= 0);
        assert!(self.clay >= 0);

        self.actions.insert(self.turn_num, BuildRobot::ObsidianRobot);
        self.wait_turns(1);
    }

    fn build_clay_robot(&mut self) {
        self.ore -= self.blueprint.clay_robot[0];
        assert!(self.ore >= 0);

        self.actions.insert(self.turn_num, BuildRobot::ClayRobot);
        self.wait_turns(1);
    }

    fn build_ore_robot(&mut self) {
        self.ore -= self.blueprint.ore_robot[0];
        assert!(self.ore >= 0);

        self.actions.insert(self.turn_num, BuildRobot::OreRobot);
        self.wait_turns(1);
    }

    fn build_next_robots(mut self) -> Vec<Self> {
        let mut rval = Vec::new();
        let bp = &self.blueprint;

        // check for geode robot
        let turns = self.turns_for_robot(&bp.geode_robot);
        if turns >= 0 {
            let mut new = self.clone();
            new.wait_turns(turns);
            new.build_geode_robot();
            rval.push(new);
        }

        // check for obsidian robot
        let turns = self.turns_for_robot(&bp.obsidian_robot);
        if turns >= 0 {
            let mut new = self.clone();
            new.wait_turns(turns);
            new.build_obsidian_robot();
            rval.push(new);
        }

        // can always wait for more ore and built clay/ore robot
        let turns = self.turns_for_robot(&bp.clay_robot);
        if turns >= 0 {
            let mut new = self.clone();
            new.wait_turns(turns);
            new.build_clay_robot();

            rval.push(new);
        }
        let turns = self.turns_for_robot(&bp.ore_robot);
        if turns >= 0 {
            let mut new = self.clone();
            new.wait_turns(turns);
            new.build_ore_robot();

            rval.push(new);
        }

        // final option: build nothing for the rest of time
        self.wait_turns(self.time_limit - self.turn_num + 1);
        rval.push(self);

        rval
    }

    fn max_geodes(&self) -> i32 {
        let turns_left = self.time_limit - self.turn_num + 1;
        let mut rval = self.geodes + turns_left * self.geode_robots;

        for new_bots in 1..turns_left {
            rval += new_bots;
        }

        rval
    }
}

fn read_file(filename: &str) -> Vec<Blueprint> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        rval.push(Blueprint::from_line(line));
    }
    rval
}

fn breadth_first_max(bp: &Blueprint, time_limit: i32) -> Strategy {
    // use a binary heap to be a depth-first search of all possible
    // strategies
    let s = Strategy::new(bp, time_limit);
    let mut strats = Vec::new();
    let mut best_strat: Option<Strategy> = None;

    strats.push(s);
    while !strats.is_empty() {
        let mut new_strats = Vec::new();

        for s in strats {
            // prune dead branches
            if false && best_strat.as_ref().is_some_and(|bs| {
                        bs.geodes >= s.max_geodes()
                    }) {
                //println!("Pruning\n{}", s);
                continue;
            }

            for new_s in s.build_next_robots() {
                if new_s.turn_num > new_s.time_limit {
                    // check for new best strategy
                    best_strat = match best_strat {
                        Some(bs) => {
                            if new_s.geodes > bs.geodes {
                                //println!("New best strategy: {}", new_s);
                                //new_s.print_strat();
                                Some(new_s)
                            } else {
                                Some(bs)
                            }
                        },
                        _ => Some(new_s)
                    };
                } else {
                    new_strats.push(new_s);
                }
            }
        }

        strats = new_strats;
    }

    best_strat.unwrap()
}

fn find_max_score(bp: &Blueprint, time_limit: i32) -> Strategy {
    // use a binary heap to be a depth-first search of all possible
    // strategies
    let s = Strategy::new(bp, time_limit);
    let mut strats = BinaryHeap::new();
    let mut best_strat: Option<Strategy> = None;

    strats.push(s);
    while !strats.is_empty() {
        let s = strats.pop().unwrap();
        //println!("Expanding: {}", s);
        //std::thread::sleep_ms(500);

        // prune dead branches
        if best_strat.as_ref().is_some_and(|bs| {
                    bs.geodes >= s.max_geodes()
                }) {
            //println!("Pruning\n{}", s);
            continue;
        }

        //for new_s in s.take_turn() {
        for new_s in s.build_next_robots() {
            if new_s.turn_num > new_s.time_limit {
                // check for new best strategy
                best_strat = match best_strat {
                    Some(bs) => {
                        if new_s.geodes > bs.geodes {
                            //println!("New best strategy: {}", new_s);
                            //new_s.print_strat();
                            Some(new_s)
                        } else {
                            Some(bs)
                        }
                    },
                    _ => Some(new_s)
                };
            } else {
                strats.push(new_s);
            }
        }
    }

    best_strat.unwrap()
}

fn part_1(mut blueprints: Vec<Blueprint>) -> i32 {
    let mut rval = 0;

    for b in blueprints.iter_mut() {
        let bs;
        println!("{}", b);
        if true {
            bs = find_max_score(&b, 24);
        } else {
            bs = breadth_first_max(&b, 24);
        }

        println!("Best strategy found {} geodes", bs.geodes);
        //bs.print_strat();
        b.best_score = bs.geodes;

        rval += b.id * b.best_score;
    }

    rval
}

fn part_2(blueprints: Vec<Blueprint>) -> i32 {
    let mut rval = 1;

    for b in blueprints.iter() {
        let bs;
        println!("{}", b);
        bs = find_max_score(&b, 32);

        println!("Best strategy found {} geodes", bs.geodes);
        rval *= bs.geodes;
    }

    rval
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let blueprints = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(blueprints.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 33);
    } else {
        assert_eq!(answer, 1389);
    }

    let now = Instant::now();
    let answer = part_2(blueprints[0..3].to_vec());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        //assert_eq!(answer, 58);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_blueprints() -> Vec<Blueprint> {
        read_file("./test")
    }

    #[test]
    fn print_test_strat() {
        let blueprints = get_blueprints();
        let mut s = Strategy::new(blueprints.get(0).unwrap(), 24);
        s.actions.insert(3, BuildRobot::ClayRobot);
        s.actions.insert(5, BuildRobot::ClayRobot);
        s.actions.insert(7, BuildRobot::ClayRobot);
        s.actions.insert(11, BuildRobot::ObsidianRobot);
        s.actions.insert(12, BuildRobot::ClayRobot);
        s.actions.insert(15, BuildRobot::ObsidianRobot);
        s.actions.insert(18, BuildRobot::GeodeRobot);
        s.actions.insert(21, BuildRobot::GeodeRobot);

        s.print_strat();
    }
}