//
// Advent of code 2022 Dec-4, part 1 and 2
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

fn fully_within(f1:i64,f2:i64,s1:i64,s2:i64) -> bool {
    return (f1>=s1 && f2 <= s2) || (s1>=f1 && s2 <= f2);
}

fn any_overlap(f1:i64,f2:i64,s1:i64,s2:i64) -> bool {
    let partial_overlap: bool = f1 >= s1 && f1 <= s2 || f2 >=s1 && f2 <= s2;
    return partial_overlap || fully_within(f1,f2,s1,s2);
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

        let mut fully_within_score = 0;
        let mut partial_overlap_score = 0;
        for s in collection {
            if s.len() > 0 {
                let numbers_on_line:Vec<&str> = s.split(&['-', ','][..]).collect();
                // Break out out for prettyness
                // The conversion to i64 could have been intermediate
                // to another array in a loop, but we still needee to break out to
                // named variables so skip that step.
                let elf1_start = numbers_on_line[0].parse::<i64>().unwrap();
                let elf1_stop  = numbers_on_line[1].parse::<i64>().unwrap();
                let elf2_start = numbers_on_line[2].parse::<i64>().unwrap();
                let elf2_stop  = numbers_on_line[3].parse::<i64>().unwrap();
            
                if fully_within(elf1_start,elf1_stop, elf2_start,elf2_stop) {
                    fully_within_score+=1;
                }
                if any_overlap(elf1_start,elf1_stop, elf2_start,elf2_stop) {
                    partial_overlap_score +=1;
                }
            } else {
                // Emtpy line, probalby LF at the end of the file
            }

        }

        println!("Fully overlapping (part 1): {}\nPartial overlap (part 2): {}", fully_within_score, partial_overlap_score)
    }
}
