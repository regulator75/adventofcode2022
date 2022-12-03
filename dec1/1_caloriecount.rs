//
// Advent of code 2022 Dec-1, part 1
//

// Reader beware, this is me learning rust.

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2  {
        println!("Usage: {} <input_filename>\n", args[0]);
    } else {
        let filename = &args[1];
        // Let the file be mutable, because we will
        // do things that changes the state, i assume
        let mut fh = match File::open(filename) {
            Err(reason) => panic!("Could not open {} : {}", filename, reason),
            Ok(fh) => fh
        };

        // Load the file into memory and chop it into lines
        let mut content: String = String::new();
        fh.read_to_string(&mut content);
        let split = content.split("\n");

        // Get ourselves some elves
        let mut elves = Vec::new();
        elves.push(0); // Assume there is always one, carrying nothing to begin with
        let mut highest_calorie_index = 0; // Assume the first elf is in the lead

        // Go through all lines in the file
        // If there are calories, add that to the current elf being
        // assessed.
        for s in split {
            if s.len() > 0 {
                let calories = s.parse::<i64>().unwrap();



                let last_elf = elves.len()-1;
                elves[last_elf] += calories;
                if elves[last_elf] > elves[highest_calorie_index] {
                    highest_calorie_index = elves.len()-1;
                }                
                // Add to the existing elf
            } else {
                // new elf
                elves.push(0);
            }
        }

        // Report
        println!("Found {} elves\n", elves.len());
        println!("Elf with most calories was #{} and she was carrying {} calories",
            highest_calorie_index,
            elves[highest_calorie_index] );

    }
}
