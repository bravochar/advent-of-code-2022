
use core::cmp::Ordering;
use std::{fmt, vec};
use std::cmp::max;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

//const FILENAME: &str = "./input";
const FILENAME: &str = "./test";
const NUM_MIN: i32 = 30;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
    paths: Vec<Vec<String>>
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tunnel_names = self.tunnels.join(", ");
        let tunnel_desc = match self.tunnels.len() {
            1 => "tunnel leads to valve ",
            _ => "tunnels lead to valves"
        };
        write!(f, "Valve {} has flow rate {}: {} {}",
            self.name,
            self.flow_rate,
            tunnel_desc,
            tunnel_names)
    }
}

impl Valve {
    fn new(name: String, flow_rate: i32, tunnels: Vec<String>) -> Valve {
        Valve{
            name,
            flow_rate,
            tunnels,
            paths: Vec::new()}
    }

    fn build_paths(&mut self, valves: &Vec<Valve>) {
        let mut visited_valves: Vec<&String> = Vec::new();
        visited_valves.push(&self.name);

        let mut vec_steps: Vec<Vec<String>> = Vec::new();

        // first steps: immediate neighbors
        for t in self.tunnels.iter() {
            let v: &Valve = valves.iter().find(|x| {
                x.name.contains(t)
            }).unwrap();
            vec_steps.push(vec![v.name.clone()]);
        }
        while !vec_steps.is_empty() {
            let mut new_steps = Vec::new();

            for cur_steps in vec_steps {
                let cur_valve = cur_steps.last().unwrap();
                let cur_valve: &Valve = valves.iter().find(|x| {
                    x.name.contains(cur_valve)
                }).unwrap();

                // check for open
                if cur_valve.flow_rate > 0 {
                    self.paths.push(cur_steps.clone());
                }

                // add new_steps
                for t in cur_valve.tunnels.iter() {
                    let next_valve = valves.iter().find(|x| {
                        x.name.contains(t)
                    }).unwrap();

                    if visited_valves.contains(&&next_valve.name) {
                        continue;
                    }
                    visited_valves.push(t);
                    let mut steps = cur_steps.clone();
                    steps.push(t.clone());
                    new_steps.push(steps);
                }
            }
            vec_steps = new_steps;
        }
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        let rval = self.flow_rate.cmp(&other.flow_rate);

        if rval == Ordering::Equal {
            self.name.cmp(&other.name)
        } else {
            rval
        }
    }
}
impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Valve { }

fn read_file(filename: &str) -> Vec<Valve> {
    let mut rval: Vec<Valve> = Vec::new();

    // Open the file
    let file = File::open(filename).unwrap();
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
        let line = line.trim();
        let (_, line) = line.trim().split_once("Valve ").unwrap();
        let (name, line) = line.split_once(" has flow rate=").unwrap();
        let (flow_rate, line) = line.split_once("; ").unwrap();
        let tunnel_iter = line.split(" ");
        let tunnels = tunnel_iter.skip(4).map(|s| {
            s.trim_end_matches(",").to_string()}).collect();

        // make vec of String
        rval.push(Valve::new(
            name.to_string(),
            flow_rate.parse().unwrap(),
            tunnels));
    }

    rval
}

#[derive(Clone, Debug)]
enum Step<'v> {
    Move(&'v Valve),
    Open(&'v Valve)
}

impl <'v> fmt::Display for Step<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Step::Move(v) => {
                    write!(f, "You move to valve {}.", v.name)
                },
                Step::Open(v) => {
                    write!(f, "You open valve {}.", v.name)
                }
            }
    }
}

#[derive(Clone, Debug)]
struct Path<'v> {
    steps: Vec<Step<'v>>,
    open_valves: Vec<&'v Valve>,
    closed_valves: Vec<&'v Valve>,
    current_flow: i32,
    total_flow: i32,
    cur_valve: &'v Valve,
    rem_time: i32,
}

impl <'v> Path<'v> {
    fn new(closed_valves: Vec<&'v Valve>, v: &'v Valve, rem_time: i32) -> Path<'v> {
        let mut p = Path {
            steps: Vec::new(),
            open_valves: Vec::new(),
            closed_valves,
            current_flow: 0,
            total_flow: 0,
            cur_valve: v,
            rem_time: rem_time + 1
        };

        p.add_step(Step::Move(v));

        p
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut rval = String::new();

        let mut open_valves: Vec<String> = Vec::new();
        let mut current_flow = 0;
        for (i, s) in self.steps.iter().enumerate() {
            rval += &format!("== Minute {} ==\n", i);
            match open_valves.len() {
                0 => rval.push_str("No valves are open\n"),
                1 => rval += &format!("Valve {} is open, releasing {} pressure.\n",
                        open_valves.first().unwrap(),
                        current_flow),
                _ => {
                    rval += &format!("Valves {:?} are open, releasing {} pressure.\n",
                        open_valves,
                        current_flow)
                }
            }
            rval += &format!("{}\n", s);
            match s {
                Step::Move(_) => {
                },
                Step::Open(v) => {
                    open_valves.push(v.name.to_owned());
                    current_flow += v.flow_rate;
                }
            }
            rval += "\n";
        }

        rval + &format!("Total_flow: {}", self.final_score())
    }

    fn open_valve(&mut self) {
        let v = self.cur_valve;
        self.closed_valves.retain(|x| {
            !&x.name.contains(&v.name)});
        self.open_valves.push(v);
        self.current_flow += v.flow_rate;
    }

    fn open_cur_valve(&mut self) {
        self.add_step(Step::Open(&self.cur_valve));
    }

    fn add_step(&mut self, s: Step<'v>) {
        self.total_flow += self.current_flow;
        match s {
            Step::Open(v) => {
                self.cur_valve = v;
                self.open_valve();
            },
            Step::Move(v) => self.cur_valve = v
        }
        self.steps.push(s);
        self.rem_time -= 1;
    }

    fn final_score(&self) -> i32 {
        self.total_flow + self.current_flow * self.rem_time
    }

    // TODO: Devise tests for more accurate ideal score - we can use the
    //      provided solutions and verify at every step of the way that their
    //      ideal score is >= their eventual score
    fn ideal_score(&self) -> i32 {
        let mut rval = self.total_flow;
        let mut flow = self.current_flow;
        let mut can_open = match self.steps.last().unwrap() {
            Step::Open(_) => false,
            _ => true
        };

        let mut j = 0;
        for _ in 0..self.rem_time {
            rval += flow;
            if can_open {
                match self.closed_valves.get(j as usize) {
                    Some(v) => flow += v.flow_rate,
                    _ => ()
                }
                j += 1;
                can_open = false;
            } else {
                can_open = true;
            }
        }
        rval
    }

    fn next_steps(&self) -> Vec<String> {
        let mut rval = Vec::new();
        if self.rem_time <= 1 {
            return rval;
        }

        let last_step = self.steps.last().unwrap();

        let v = match last_step {
            Step::Move(v) => {
                v
            }, 
            Step::Open(v) => v
        };

        for t in v.tunnels.iter() {
            rval.push(t.clone());
        }

        rval
    }

    #[allow(dead_code)]
    fn path_len(&self) -> i32 {
        return self.steps.len() as i32 - 1
    }

    fn can_open(&self) -> bool {
        let v = self.cur_valve;

        match self.steps.last().unwrap() {
            Step::Open(_) => return false,
            _ => ()
        }

        for c_v in self.closed_valves.iter() {
            if c_v.name.contains(&v.name) {
                return true;
            }
        }
        false
    }

    fn all_open(&self) -> bool {
        self.closed_valves.is_empty()
    }

    // TODO: need add_step, cmp, and best_move_n
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ideal_score() {
        let valves = read_file("./test");

        // hashmap to find next move
        let mut valve_map = HashMap::new();
        let mut closed_valves = Vec::new();
        for v in valves.iter() {
            valve_map.insert(v.name.clone(), v);
            if v.flow_rate > 0 {
                closed_valves.push(v);
            }
            //println!("{}", v);
        }

        closed_valves.sort_by(|a, b| {
            b.flow_rate.cmp(&a.flow_rate)
        });
        println!("Need to open {} valves...", closed_valves.len());
        let final_score = 1651;
        let first_step = valve_map.get("AA").unwrap();

        let mut p = Path::new(closed_valves, &first_step, NUM_MIN);
        p.add_step(Step::Move(valve_map.get("DD").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("CC").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("BB").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        println!("with {} valves left: {} vs. {}",
            p.closed_valves.len(),
            p.ideal_score(),
            final_score);
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("AA").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("II").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("JJ").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        println!("with {} valves left: {} vs. {}",
            p.closed_valves.len(),
            p.ideal_score(),
            final_score);
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("II").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("AA").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("DD").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("EE").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("FF").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("GG").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("HH").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        println!("with {} valves left: {} vs. {}",
            p.closed_valves.len(),
            p.ideal_score(),
            final_score);
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("GG").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("FF").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("EE").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        println!("with {} valves left: {} vs. {}",
            p.closed_valves.len(),
            p.ideal_score(),
            final_score);
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("DD").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.add_step(Step::Move(valve_map.get("CC").unwrap()));
        assert!(p.ideal_score() >= final_score);
        p.open_cur_valve();
        println!("with {} valves left: {} vs. {}",
            p.closed_valves.len(),
            p.ideal_score(),
            final_score);
        assert!(p.ideal_score() >= final_score);

    }
}

fn widen_path<'v>(p: Path<'v>,
        mut visited_valves: Vec<&'v Valve>,
        valve_map: &HashMap<String, &'v Valve>) ->
            (Vec<Path<'v>>, Vec<&'v Valve>) {

    let mut new_steps: Vec<Path<'v>> = Vec::new();

    for v_name in p.next_steps().iter() {
        if visited_valves.iter().any(|v| {v.name.eq(v_name)}) {
            continue;
        } else if p.rem_time < 1 {
            continue;
        }
        let v = valve_map.get(v_name).unwrap();
        visited_valves.push(v);
        let mut new_path = p.clone();
        new_path.add_step(Step::Move(v));

        new_steps.push(new_path);
    }

    (new_steps, visited_valves)
}

fn find_best_path<'v>(
        p: Path<'v>,
        valve_map: &HashMap<String, &'v Valve>) -> Path<'v> {
    return _find_best_path(0, p.clone(), valve_map, p);
}

fn open_next_valves<'v>(
        p: Path<'v>,
        valve_map: &HashMap<String, &'v Valve>,
            ) -> Vec<Path<'v>> {
    let mut poss_paths: Vec<Path> = Vec::new();

    poss_paths.push(p.clone());
    let mut opened_paths: Vec<Path> = Vec::new();

    for str_path in p.cur_valve.paths.iter() {
        let to_open_name = str_path.last().unwrap();
        if !p.closed_valves.iter().any( |v| {
                v.name.contains(to_open_name)}) {
            continue;
        }
        
        let mut new_path = p.clone();
        for valve_name in str_path.iter() {
            new_path.add_step(
                Step::Move(valve_map.get(valve_name).unwrap()));
        }
        new_path.open_cur_valve();
        opened_paths.push(new_path);
    }
    /*
    while !poss_paths.is_empty() {
        let mut new_steps: Vec<Path> = Vec::new();
        for p in poss_paths.into_iter() {
            if p.ideal_score() < best_score {
                continue;
            }

            if p.can_open() {
                let mut new_path = p.clone();
                new_path.open_cur_valve();

                opened_paths.push(new_path);
            }

            let my_steps;
            (my_steps, visited_valves) = widen_path(p, visited_valves, valve_map);
            new_steps.extend(my_steps);
        }

        poss_paths = new_steps;
    }
     */

    opened_paths
}

fn _find_best_path<'v>(
        level: i32,
        p: Path<'v>,
        valve_map: &HashMap<String, &'v Valve>,
        mut best_path: Path<'v>)
            -> Path<'v> {
    let mut poss_paths: Vec<Path> = Vec::new();
    let mut visited_valves: Vec<&Valve> = Vec::new();

    if p.final_score() > best_path.final_score() {
        /*
        println!("{}: New best path: {} ({} valves left; {} time) (was {})",
            level,
            p.final_score(),
            p.closed_valves.len(),
            p.rem_time,
            best_path.final_score());
         */
        best_path = p.clone();
    }

    // no more recursing
    if p.all_open() {
        return best_path;
    }

    poss_paths.push(p.clone());
    let mut opened_paths: Vec<Path> = Vec::new();
    if false {
        while !poss_paths.is_empty() {
            let mut new_steps: Vec<Path> = Vec::new();
            for p in poss_paths.into_iter() {
                if p.ideal_score() < best_path.final_score() {
                    continue;
                }

                if p.can_open() {
                    let mut new_path = p.clone();
                    new_path.open_cur_valve();

                    opened_paths.push(new_path);
                }

                let my_steps;
                (my_steps, visited_valves) = widen_path(p, visited_valves, valve_map);
                new_steps.extend(my_steps);
            }

            poss_paths = new_steps;
        }
    } else {
        opened_paths = open_next_valves(p, valve_map);
    }

    // sort to explore most promising branches first
    opened_paths.sort_by(|a, b| {
        b.ideal_score().cmp(&a.ideal_score())
    });
    for p in opened_paths {
        if p.ideal_score() < best_path.final_score() {
            break;
        }
        best_path = _find_best_path(
            level + 1,
            p,
            valve_map,
            best_path);
    }

    best_path
}

fn part_1(valves: Vec<Valve>) {

    // hashmap to find next move
    let mut valve_map = HashMap::new();
    let mut closed_valves = Vec::new();
    for v in valves.iter() {
        valve_map.insert(v.name.clone(), v);
        if v.flow_rate > 0 {
            closed_valves.push(v);
        }
        //println!("{}", v);
    }

    println!("Need to open {} valves...", closed_valves.len());
    closed_valves.sort_by(|a, b| {
        b.flow_rate.cmp(&a.flow_rate)
    });
    for v in closed_valves.iter() {
        println!("  {}: {}", v.name, v.flow_rate);
    }
    let first_step = valve_map.get("AA").unwrap();

    let p = Path::new(closed_valves.clone(), &first_step, NUM_MIN);
    
    let best_path = find_best_path(p, &valve_map);

    println!("Best path of {} steps: {}",
        best_path.steps.len(),
        best_path.final_score());
    
    if FILENAME.eq("./test") {
        assert_eq!(best_path.final_score(), 1651);
    } else {
        assert_eq!(best_path.final_score(), 1673);
    }
    //println!("{}", best_path.to_string());
}

#[derive(Clone, Debug)]
struct DuplexPath<'v> {
    my_path: Path<'v>,
    elephant_path: Path<'v>
}

impl <'v> DuplexPath<'v> {
    fn new(closed_valves: Vec<&'v Valve>, v: &'v Valve, rem_time: i32) -> DuplexPath<'v> {
        let mut p = Path {
            steps: Vec::new(),
            open_valves: Vec::new(),
            closed_valves,
            current_flow: 0,
            total_flow: 0,
            cur_valve: v,
            rem_time: rem_time + 1
        };
        p.add_step(Step::Move(v));

        DuplexPath {
            my_path: p.clone(),
            elephant_path: p.clone()
        }
    }

    fn closed_valves(&self) -> i32 {
        self.my_path.closed_valves.len() as i32
    }

    fn rem_time(&self) -> i32 {
        if self.my_path.rem_time > self.elephant_path.rem_time {
            self.elephant_path.rem_time
        } else {
            self.my_path.rem_time
        }
    }

    #[allow(dead_code)]
    fn flow_rate(&self) -> i32 {
        self.my_path.current_flow + self.elephant_path.current_flow
    }

    fn final_score(&self) -> i32 {
        self.my_path.final_score() + self.elephant_path.final_score()
    }

    fn ideal_score(&self) -> i32 {
        // first approximation: final score of longer path plus ideal score of
        // the shorter path
        if self.my_path.rem_time > self.elephant_path.rem_time {
            self.elephant_path.final_score() + self.my_path.ideal_score()
        } else {
            self.my_path.final_score() + self.elephant_path.ideal_score()
        }
    }

    fn steps(&self) -> i32 {
        max(self.my_path.steps.len(),
                self.elephant_path.steps.len()) as i32 - 1
    }

    #[allow(dead_code)]
    fn all_open(&self) -> bool {
        self.my_path.all_open() && self.elephant_path.all_open()
    }

    fn open_my_path(&mut self) {
        self.my_path.open_cur_valve();
        self.elephant_path.closed_valves = self.my_path.closed_valves.clone();
    }

    fn open_elephant_path(&mut self) {
        self.elephant_path.open_cur_valve();
        self.my_path.closed_valves = self.elephant_path.closed_valves.clone();
    }
}

fn find_best_path_duplex<'v>(
        level: i32,
        in_path: DuplexPath<'v>,
        valve_map: &HashMap<String, &'v Valve>)
            -> DuplexPath<'v> {
    let best_path = in_path.clone();
    return _find_best_path_duplex(level, in_path, valve_map, best_path);
}

fn _find_best_path_duplex<'v>(
        level: i32,
        in_path: DuplexPath<'v>,
        valve_map: &HashMap<String, &'v Valve>,
        mut best_path: DuplexPath<'v>)
            -> DuplexPath<'v> {
    let mut poss_paths: Vec<DuplexPath> = Vec::new();
    let mut my_visited: Vec<&Valve> = Vec::new();
    let mut elephant_visited: Vec<&Valve> = Vec::new();

    if in_path.final_score() > best_path.final_score() {
        println!("{}: New best path: {} ({} valves left; {} time) (was {})",
            level,
            in_path.final_score(),
            in_path.closed_valves(),
            in_path.rem_time(),
            best_path.final_score());
        best_path = in_path.clone();
    }

    poss_paths.push(in_path.clone());
    let mut opened_paths: Vec<DuplexPath> = Vec::new();
    if false {
        while !poss_paths.is_empty() {
            let mut new_steps: Vec<DuplexPath> = Vec::new();
            for p in poss_paths.into_iter() {
                if p.ideal_score() < best_path.final_score() {
                    continue
                }

                if p.my_path.can_open() {
                    let mut new_path = DuplexPath {
                        my_path: p.my_path.clone(),
                        elephant_path: in_path.elephant_path.clone()
                    };
                    new_path.open_my_path();

                    opened_paths.push(new_path);
                }

                if p.elephant_path.can_open() {
                    let mut new_path = DuplexPath {
                        my_path: in_path.my_path.clone(),
                        elephant_path: p.elephant_path.clone()
                    };
                    new_path.open_elephant_path();
                    
                    opened_paths.push(new_path);
                }

                if p.elephant_path.rem_time == in_path.elephant_path.rem_time {
                    let my_paths;
                    (my_paths, my_visited) = widen_path(
                        p.my_path.clone(),
                        my_visited,
                        valve_map);
                    
                    for p in my_paths {
                        new_steps.push(DuplexPath {
                            my_path: p,
                            elephant_path: in_path.elephant_path.clone()
                        });
                    }
                }

                if p.my_path.rem_time == in_path.my_path.rem_time {
                    let elephant_paths;
                    (elephant_paths, elephant_visited) = widen_path(
                        p.elephant_path.clone(),
                        elephant_visited,
                        valve_map);
                    
                    for p in elephant_paths {
                        new_steps.push(DuplexPath {
                            my_path: in_path.my_path.clone(),
                            elephant_path: p
                        });
                    }
                }
            }
            poss_paths = new_steps;
        }

    } else {
        let elephant_paths = open_next_valves(
            in_path.elephant_path.clone(),
            valve_map);
        for p in elephant_paths {
            let mut new_path = DuplexPath {
                my_path: in_path.my_path.clone(),
                elephant_path: p
            };
            new_path.my_path.closed_valves = new_path.elephant_path.closed_valves.clone();
            opened_paths.push(new_path);
        }

        let my_paths = open_next_valves(
            in_path.my_path.clone(),
            valve_map);
        for p in my_paths {
            let mut new_path = DuplexPath {
                my_path: p,
                elephant_path: in_path.elephant_path.clone()
            };
            new_path.elephant_path.closed_valves = new_path.my_path.closed_valves.clone();
            opened_paths.push(new_path);
        }
    }

    // sort to explore most promising branches first
    opened_paths.sort_by(|a, b| {
        b.ideal_score().cmp(&a.ideal_score())
    });
    for d in opened_paths {
        if d.ideal_score() < best_path.final_score() {
            break;
        }
        best_path = _find_best_path_duplex(
            level + 1,
            d,
            valve_map,
            best_path);
    }

    best_path
}

fn part_2(valves: Vec<Valve>) {
    /*
     * same as above, but with a 26 minute limit, and two actors.
     * 
     * We can make our possible solutions a tuple of two paths. Take turns
     * permuting each element of the tuple, synchronizing the closed valves
     * between the two paths. The score is then a combination of the individual
     * scores, and the ideal score should be calculated at a higher level?
     * 
     * Maybe use a struct to hold the two paths and re-implement the scoring
     * methods there, i.e. the ideal score starts with the smaller of the two
     * paths until the two catch up. And we should permute the smaller of the 
     * two paths at each turn, not necessarily alternate?
     */

    // hashmap to find next move
    let mut valve_map = HashMap::new();
    let mut closed_valves = Vec::new();
    for v in valves.iter() {
        valve_map.insert(v.name.clone(), v);
        if v.flow_rate > 0 {
            closed_valves.push(v);
        }
    }

    closed_valves.sort_by(|a, b| {
        b.flow_rate.cmp(&a.flow_rate)
    });
    let first_step = valve_map.get("AA").unwrap();

    println!("\nPart 2 - Opening {} closed valves:", closed_valves.len());

    let p = DuplexPath::new(
        closed_valves.clone(),
        &first_step,
        NUM_MIN - 4);
    
    let best_path = find_best_path_duplex(0, p, &valve_map);
    /*
    let mut high_score = 0;
    let mut paths = vec![p];
    while !paths.is_empty() {
        let mut next_paths: Vec<DuplexPath> = Vec::new();
        for path in paths.into_iter() {
            if path.ideal_score() <= high_score {
                continue;

            } else if path.final_score() > high_score {
                high_score = path.final_score();
                best_path = path.clone();
            }

            let new_paths = permute_duplex_path(path, &valve_map);

            if !new_paths.is_empty() {
                next_paths.extend(new_paths);
            }
        }

        paths = Vec::new();
        for path in next_paths.into_iter() {
            // check for new high score
            if path.final_score() > high_score {
                high_score = path.final_score();
                best_path = path.clone();
            }

            // check pruning conditions
            if path.ideal_score() < high_score {
                continue;
            } else if path.all_open() {
                continue;
            } else {
                paths.push(path);
            }
        }
        if !paths.is_empty() {
            println!("Working with {} paths; best {}", paths.len(), high_score);
        } else {
            println!("Done!");
        }
    }
     */

    println!("Best path of {} steps: {}",
        best_path.steps(),
        best_path.final_score());
    if FILENAME.eq("./test") {
        assert_eq!(best_path.final_score(), 1707);
    }
    
}

fn main() {
    println!("Advent of Code, Day 16");

    let mut valves = read_file(FILENAME);
    let orig_valves = valves.clone();
    for v in valves.iter_mut() {
        v.build_paths(&orig_valves);
    }

    let now = Instant::now();
    use std::time::Instant;
    part_1(valves.clone());
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);

    let now = Instant::now();
    part_2(valves.clone());
    let elapsed = now.elapsed();
    println!("Took {:.5?}", elapsed);
}
