//
// Advent of code 2022 Dec-6, part 1
//

// Reader beware, this is me learning rust.

use std::env;
use std::fs::File;
use std::io::prelude::*;

const START_OF_PACKAGE_MARKER_LENGTH: usize = 4;
const START_OF_MESSAGE_MARKER_LENGTH: usize = 14;


//
// Loads a file into the supplied "data" parameter. Returns an iterator
// over the content of the file, each element represent one line. 
//
fn load_file<'a>(filename: &String, data: &'a mut String){
    // Let the file be mutable, because we will
    // do things that changes the state, i assume
    let mut fh = match File::open(filename) {
        Err(reason) => panic!("Could not open {} : {}", filename, reason),
        Ok(fh) => fh
    };

    // Load the file into memory and chop it into lines
    match fh.read_to_string(data) {
        Err(reason) => panic!("Could not read content from {}: {}", filename, reason),
        Ok(x) => x
    };
}

// return strue if this is a start-of-buffer marker
fn process_character(buffer : &mut String, marker_length : usize, latest : char) -> bool {

    // Remove character from buffer until all elements
    // would be unique after 'latest' is added
    while buffer.find(latest) != None && buffer.len() > 0 {
        buffer.pop();
    }

    buffer.insert(0,latest);

    return buffer.len() == marker_length;
}

fn main() {
    let mut file_content = String::new();
    let args:Vec<String> = env::args().collect();
    load_file(&args[1], &mut file_content);
    let mut packet_marker_buffer: String = String::new();

    for (idx, c) in file_content.chars().enumerate() { 
        if process_character(&mut packet_marker_buffer, START_OF_PACKAGE_MARKER_LENGTH, c) {
            println!("start-of-packet marker {}", idx+1);
            break; // Do not continue
        }
    }

    let mut message_marker_buffer: String = String::new();
    for (idx, c) in file_content.chars().enumerate() { 
        if process_character(&mut message_marker_buffer, START_OF_MESSAGE_MARKER_LENGTH, c) {
            println!("start-of-message marker {}", idx+1);
            break; // Do not continue
        }
    }

} 
