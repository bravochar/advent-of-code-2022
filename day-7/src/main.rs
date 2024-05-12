use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";

#[derive(Debug)]
struct DirEntry {
    path: String,
    file_size: usize,
    is_dir: bool,
    entries: Vec<DirEntry>,
}

impl DirEntry {
    fn size(&self) -> usize {
        if !self.is_dir {
            self.file_size
        } else {
            let mut rval = 0;
            for e in self.entries.iter() {
                rval += e.size();
            }

            rval
        }
    }

    #[allow(dead_code)]
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }

    #[allow(dead_code)]
    fn is_path(&self, path: &str) -> bool {
        if self.path.eq(path) {
            return true;
        }

        false
    }

    fn new_file(path: &str, file_size: usize) -> Self {
        Self {
            path: String::from(path),
            file_size,
            is_dir: false,
            entries: Vec::<DirEntry>::new(),
        }
    }

    fn new_dir(path: &str) -> Self {
        let mut dir_path = String::from(path);
        if !dir_path.ends_with("/") {
            dir_path.push('/');
        }
        Self {
            path: dir_path,
            file_size: 0,
            is_dir: true,
            entries: Vec::<DirEntry>::new(),
        }
    }

    fn add_dir(&mut self, dir: &mut Self) -> bool {
        if self.is_dir {
            if self.path.eq(&dir.path) {
                while !dir.entries.is_empty() {
                    self.entries.insert(0, dir.entries.pop()
                        .expect("This shouldn't ever happen"));
                }
                return true;
            }

            for i in 0..self.entries.len() {
                if self.entries[i].add_dir(dir) {
                    return true;
                }
            }
        }

        false
    }

    fn print_dir(d: &Self, prefix: &str) {
        for e in d.entries.iter() {
            if e.is_dir {
                let mut new_prefix = String::from("  ");
                new_prefix.push_str(prefix);

                println!("{}- {} (dir)", prefix, e.path);
                DirEntry::print_dir(&e, &new_prefix);

            } else {
                println!("{}- {} (file, size={})", prefix, e.path, e.size());
            }
        }
    }

    fn print(&self) {
        println!("- {} (dir)", self.path);

        DirEntry::print_dir(&self, "  ");
    }
}

fn part_1() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // track current directory
    let mut pwd = String::new();
    let mut proc_output = false;
    let mut root = DirEntry::new_dir("/");
    let mut cur_dir = DirEntry::new_dir("/");

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let line_str = line_str.trim();

        if line_str.starts_with("$ cd ") {

            // insert previously listed directory into tree
            if proc_output {
                if !root.add_dir(&mut cur_dir) {
                    println!("Error: couldn't add dir {:?}", cur_dir);
                    return;
                }
                cur_dir = DirEntry::new_dir("/");
            }
            proc_output = false;

            if line_str.len() < 6 {
                // invalid `cd` - no change to state
                continue;
            }
            let dir_str = &line_str[5..];
            let dir_str = dir_str.trim();

            if dir_str.starts_with("/") {
                pwd.clear();
                pwd.push_str(dir_str);

            } else if dir_str.starts_with("..") {
                if pwd.len() < 2 {
                    // no parent directories here!
                    continue;
                }

                // remove trailing slash
                if pwd.ends_with("/") {
                    pwd.pop();
                }

                // find previous directory marker
                let par_dir = pwd.rfind("/");

                // truncate string to marker from above
                if let Some(dir_mark) = par_dir {
                    pwd.truncate(dir_mark + 1);
                }

            } else {
                pwd.push_str(dir_str);
                pwd.push_str("/");
            }

        } else if line_str.starts_with("$ ls") {
            proc_output = true;
            println!("ls called for: {}", pwd);

            cur_dir = DirEntry::new_dir(&pwd);

        } else if proc_output {
            let splits: Vec<&str> = line_str.split(" ").collect();
            if splits.len() != 2 {
                println!("Error splitting LS output: {}", line_str);
                return;
            }
            let mut path = String::from(&pwd);

            if line_str.starts_with("dir ") {
                path.push_str(splits[1]);

                let d = DirEntry::new_dir(&path);
                println!("  {:?}", d);

                cur_dir.entries.push(d);

            } else {
                let file_size = splits[0].parse().expect("Couldn't parse file size");
                path.push_str(splits[1]);

                let f = DirEntry::new_file(&path, file_size);
                println!("  {:?}", f);

                cur_dir.entries.push(f);
            }

        } else {
            // invalid state
            println!("Error: Invalid state: Exiting. Line was : '{}'", line_str);
            return;
        }
    }

    // XXX: Catch last directory here
    if proc_output {
        if !root.add_dir(&mut cur_dir) {
            println!("Error: couldn't add dir {:?}", cur_dir);
            return;
        }
    }

    for e in root.entries.iter() {
        println!("Entry {}: {} bytes", e.path, e.size());
    }

    println!("Root size is {}", root.size());
    root.print();

    // XXX: Well then, traverse the tree and save the size of all directories less than 100000
    // bytes
    let size_limit = 100000;
    let answer = sum_dirs_less_than_size(&root, size_limit);

    // Print the answer to the first part
    println!("First Answer: {:?}", answer);
}

fn sum_dirs_less_than_size(d: &DirEntry, sz_limit: usize) -> usize {
    let mut rval: usize = 0;

    for e in d.entries.iter() {
        if e.is_dir {
            if e.size() < sz_limit {
                rval += e.size();
            }

            rval += sum_dirs_less_than_size(e, sz_limit);
        }
    }

    return rval;
}

fn part_2() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // track current directory
    let mut pwd = String::new();
    let mut proc_output = false;
    let mut root = DirEntry::new_dir("/");
    let mut cur_dir = DirEntry::new_dir("/");

    // Read file line by line
    for line in reader.lines() {
        let line_str = line.expect("Couldn't read line as string?");
        let line_str = line_str.trim();

        if line_str.starts_with("$ cd ") {

            // insert previously listed directory into tree
            if proc_output {
                if !root.add_dir(&mut cur_dir) {
                    println!("Error: couldn't add dir {:?}", cur_dir);
                    return;
                }
                cur_dir = DirEntry::new_dir("/");
            }
            proc_output = false;

            if line_str.len() < 6 {
                // invalid `cd` - no change to state
                continue;
            }
            let dir_str = &line_str[5..];
            let dir_str = dir_str.trim();

            if dir_str.starts_with("/") {
                pwd.clear();
                pwd.push_str(dir_str);

            } else if dir_str.starts_with("..") {
                if pwd.len() < 2 {
                    // no parent directories here!
                    continue;
                }

                // remove trailing slash
                if pwd.ends_with("/") {
                    pwd.pop();
                }

                // find previous directory marker
                let par_dir = pwd.rfind("/");

                // truncate string to marker from above
                if let Some(dir_mark) = par_dir {
                    pwd.truncate(dir_mark + 1);
                }

            } else {
                pwd.push_str(dir_str);
                pwd.push_str("/");
            }

        } else if line_str.starts_with("$ ls") {
            proc_output = true;
            println!("ls called for: {}", pwd);

            cur_dir = DirEntry::new_dir(&pwd);

        } else if proc_output {
            let splits: Vec<&str> = line_str.split(" ").collect();
            if splits.len() != 2 {
                println!("Error splitting LS output: {}", line_str);
                return;
            }
            let mut path = String::from(&pwd);

            if line_str.starts_with("dir ") {
                path.push_str(splits[1]);

                let d = DirEntry::new_dir(&path);
                println!("  {:?}", d);

                cur_dir.entries.push(d);

            } else {
                let file_size = splits[0].parse().expect("Couldn't parse file size");
                path.push_str(splits[1]);

                let f = DirEntry::new_file(&path, file_size);
                println!("  {:?}", f);

                cur_dir.entries.push(f);
            }

        } else {
            // invalid state
            println!("Error: Invalid state: Exiting. Line was : '{}'", line_str);
            return;
        }
    }

    // XXX: Catch last directory here
    if proc_output {
        if !root.add_dir(&mut cur_dir) {
            println!("Error: couldn't add dir {:?}", cur_dir);
            return;
        }
    }

    // XXX: Well then, traverse the tree and save the size of all directories less than 100000
    // bytes
    let space_needed = 30000000;
    let total_size = 70000000;
    let size_limit = space_needed - (total_size - root.size());
    println!("Looking for {} bytes of space", size_limit);
    let answer = find_dir_closest_to_size(&root, size_limit, root.size());

    // Print the answer to the second part
    println!("Second Answer: {:?}", answer);
}

fn find_dir_closest_to_size(d: &DirEntry, sz_limit: usize, cur_lead: usize) -> usize {
    let mut rval: usize = cur_lead;

    for e in d.entries.iter() {
        if e.is_dir {
            if e.size() >= sz_limit && e.size() < rval {
                rval = e.size();

                println!("Dir {} took lead with size {}", e.path, rval);
            }

            rval = find_dir_closest_to_size(e, sz_limit, rval);
        }
    }

    return rval;
}

fn main() {
    println!("Advent of Code, Day 7");

    part_1();
    part_2();
}

