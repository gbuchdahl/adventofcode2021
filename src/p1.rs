use std::fs;
use std::io::{BufRead, BufReader};

pub fn solve() {
    // --snip--
    let filename = "inputs/p1.txt";

    let mut increases = 0;
    let mut a;
    let mut b = 2147483647;
    let mut c = 2147483647;
    let mut prev: i64 = 100000000000;

    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.

        let val = line.parse().expect("Please pass in a number!");

        a = b;
        b = c; 
        c = val;

        if a + b + c > prev {
            increases+=1;
        }

        prev = a + b + c;
    }

    println!("{} Increases", increases);
}

