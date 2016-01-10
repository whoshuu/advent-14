extern crate advent_14;

use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn find_max<'a, T: Ord>(lst: &'a Vec<T>) -> Option<&'a T> {
    let mut max = None;

    let find_max = |i: &'a T| {
        max = match max {
            None => Some(i),
            Some(ref m) if i > *m => Some(i),
            _ => max
        };
        max
    };
    lst.iter().map(find_max).last().unwrap()
}

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {},
    }

    let mut max: u32 = 0;
    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }
        let (speed, fly, rest) = advent_14::parse_string(&line.to_string());
        max = cmp::max(max, advent_14::total_distance(speed, fly, rest, 2503));
    }

    println!("Max: {}", max);

    let mut points = vec![0; 9];
    let mut reindeer = vec![0; 9];
    for seconds in 1..2504 {
        for (i, line) in s.split("\n").enumerate() {
            if line.is_empty() {
                continue;
            }
            let (speed, fly, rest) = advent_14::parse_string(&line.to_string());
            reindeer[i] = advent_14::total_distance(speed, fly, rest, seconds);
        }

        let m = find_max(&reindeer).expect("Need a max");
        for (i, r) in reindeer.iter().enumerate() {
            if r == m {
                points[i] += 1;
            }
        }
    }

    println!("Highest points: {}", find_max(&points).expect("Need a max"));
}
