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

fn prepare_for_parsing<'a>(file_content_goes_here: &'a mut String)-> std::str::Split<'a, char> {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2  {
        panic!("Usage: {} <input_filename>\n", args[0]);
    } else {
        let filename = &args[1].to_string();
        return load_file_to_line_vector(filename,file_content_goes_here);
    }
}

fn main() {
    let mut file_content = String::new();
    let file_content_per_line = prepare_for_parsing( &mut file_content ); 

    // Build a vector of vectors. I.e. a vector of the stacks.
    //
    let mut stacks : Vec<Vec<char>> = Vec::<Vec<char>>::new();

    //
    // First part, load the stack data
    // 
    // Example what it looks like
    //
    //     [A]
    // [B] [Z]
    // [S] [G] [W]
    //  1   2   3 
    //

    let mut sitenum = file_content_per_line.enumerate();

    let mut sit = sitenum.next();
    while sit != None {
        let s = sit.unwrap().1;
    //for s in file_content_per_line {
        if s.len() == 0 {
            break;
        }
        // keep track of what stack we are looking at.
        let mut stackindex = 0;
        let mut charit = s.chars();


        let mut nextchar_container = charit.next();

        // This loop assumes that nextchar_container has just read the first
        // character of a space that could be a stack.
        // Per example above, marked here with * are the columns that would be valid
        // while parsing each row
        // 
        // *   *  
        //     [A]
        //
        // *   *   *   *
        // [B] [Z]     [G]
        //
        // *   *   *   *
        // [S] [G] [W] [U]        

        while nextchar_container != None {
            let nextchar = nextchar_container.unwrap();
            //println!("Looping, character is {}", nextchar);

            if nextchar == '[' { // There is something in this stack. Extract it
                
                let data = charit.next().unwrap();
                let closebracket = charit.next().unwrap();
                if closebracket != ']' {
                    panic!("Parsing error! Read{} where there should be a '['", closebracket);
                }
                while stacks.len() < stackindex+1 {
                    // Yet another stack, lets create space for it.
                    stacks.push(vec![]);
                    println!("Adding another stack, len is now {}", stacks.len());
                }
    
                // Final step, store the information
                println!("Pushing {} onto stack {}", data,stackindex);
                stacks[stackindex].push(data);
                stackindex+=1;

                // Now, we need to leave the iterator at a 
                // valid beginning of a stack-data boundary.
                // if there are other stacks after us, there will
                // be an empty space here. If we are the last stack on this line,
                // it will be empty.
                nextchar_container = charit.next();
                if nextchar_container != None {
                    // throw away the separating space.
                    nextchar_container = charit.next();
                }

            } else if nextchar == ' ' { // Emty slot, just read three spaces
                println!("No data for stack {}", stackindex);
                // rustc bug? If I use charit.skip(3) here instead
                // I get a compiler error on the statement 
                //     let data = charit.next().unwrap();
                // above.
                charit.next();
                charit.next();
                charit.next();
                stackindex+=1;
                nextchar_container = charit.next();
            }   
        }
        sit = sitenum.next();
    }

    for st in stacks {
        print!("Stack ");
        for ch in st {
            print!("{}", ch);
        }
        println!("\n");
    }
} 
