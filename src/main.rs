extern crate advent_14;

use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

    let split = s.split("\n");
    let mut max: u32 = 0;
    for line in split {
        if line.is_empty() {
            continue;
        }
        let (speed, fly, rest) = advent_14::parse_string(&line.to_string());
        max = cmp::max(max, advent_14::total_distance(speed, fly, rest, 2503));
    }

    println!("Max: {}", max);
}
