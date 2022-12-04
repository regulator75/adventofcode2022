//
// Advent of code 2022 Dec-3, part 2
//

// Reader beware, this is me learning rust.

use std::env;
use std::fs::File;
use std::io::prelude::*;


//
// Loads a file into the supplied "data" parameter. Returns an iterator
// over the content of the file, each element represent one line. 
//
fn load_file_to_line_vector<'a>(filename: &String, data: &'a mut String) -> std::str::Split<'a, char>{
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


//
// Calculate priority for an item as described in the challenge
//
// "Lowercase item types a through z have priorities 1 through 26.
//  Uppercase item types A through Z have priorities 27 through 52."
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

        let mut score = 0;
        let mut iter = collection.peekable();

        while iter.peek() != None {
            //Bluntly assume that the file is not currupt, and read
            // the next three rucksack contents.
            // This could have been treated as an array if we where
            // unsure of team-size (not 3). but this is advent of 
            // code :-)
            let elf1_rucksack = iter.next().unwrap();
            let elf2_rucksack = iter.next().unwrap();
            let elf3_rucksack = iter.next().unwrap();

            // Find an character that is in all three
            for c in elf1_rucksack.chars() {
                if elf2_rucksack.contains(c) && elf3_rucksack.contains(c) {
                    score += score_item(c);
                    break; // Dont search further. Finding the same item would inflate score
                }
            }
        }


        println!("Score: {}", score)
    }
}
