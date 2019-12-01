// utility
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
//use std::vec;

// read by line, returning a vector of Strings
pub fn util_fread(file_path:&str) -> std::vec::Vec<String>{
	let mut vec:Vec<String> = Vec::new();
	
	let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
		vec.push(line);
    }
	
	return vec;
}