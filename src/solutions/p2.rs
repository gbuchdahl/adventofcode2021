use std::fs;
use std::io::{BufRead, BufReader};

pub fn solve() {
    // --snip--
    let filename = "input.txt";

    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let split = line.split(' ');
        let vec = split.collect::<Vec<&str>>();
        let amount: usize = vec[1].parse().expect("Expected a number");

        let dir : &str  = vec[0];

        if dir == "forward"  {
            pos += amount;
            depth += amount * aim;
        } else if dir == "down" {
            aim += amount;
        } else if dir == "up" {
            aim -= amount;
        } else {
            println!("Error!");
            return;
        }

    }
    println!("Result: {}", pos * depth);
}
