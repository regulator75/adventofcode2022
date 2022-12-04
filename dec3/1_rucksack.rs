//
// Advent of code 2022 Dec-3, part 1
//

// Reader beware, this is me learning rust.

use std::env;
use std::fs::File;
use std::io::prelude::*;

//fn load_file_to_line_vector<'a>(filename: &String) -> std::str::Split<'a, char>{ // return-type figured out by compiler
fn load_file_to_line_vector<'a>(filename: &String, data: &'a mut String) -> std::str::Split<'a, char>{ // return-type figured out by compiler
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
        return data.split('\n');
}


// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn score_item(item:char) -> u32 {
    match item {
        'a'..='z'=> item as u32 -'a' as u32 +1,
        'A'..='Z'=> item as u32 -'A' as u32 +27,
        _ => panic!("Input file included bad values")
    }
}


fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2  {
        println!("Usage: {} <input_filename>\n", args[0]);
    } else {
        let filename = &args[1].to_string();
        let mut file_content = String::new(); // Need this here becase the collection
                                              // returned by .split (and returned)
                                              // needs to point to data that does not go 
                                              // out of scope.

        let collection = load_file_to_line_vector(filename,&mut file_content);

        // Get ourselves some elves
        let mut elves = Vec::new();
        elves.push(0); // Assume there is always one, carrying nothing to begin with
        let mut score = 0;
        for s in collection {
            if s.len() > 0 {
                if s.len() % 2 == 1  {
                    panic!("Invalid input, odd number of items");
                }
                let (leftcompartment,rightcompartment) = s.split_at(s.len()/2);
                println!("Left, right: {}, {}", leftcompartment, rightcompartment);
                for cl in leftcompartment.chars() {
                    if rightcompartment.contains(cl) {
                        // Double packed! Lets call this out
                        println!("Found a double packed item: {}, scores as {}", cl, score_item(cl));
                        score += score_item(cl);
                        break;
                    }
                }
            }
        }
        println!("Score: {}", score)
    }
}
