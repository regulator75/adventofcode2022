//
// Advent of code 2022 Dec-5, part 1 and 2
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

fn prepare_for_parsing<'a>(file_content_goes_here: &'a mut String)-> (std::str::Split<'a, char>, i64) {
    let args:Vec<String> = env::args().collect();
    if args.len() != 3  {
        panic!("Usage: {} [CraneMover9000 | CraneMover9001] <input_filename>\n", args[0]);
    } else {
        let protocol_goes_here = match args[1].to_string().as_str() {
            "CraneMover9000" => 1,
            "CraneMover9001" => 2,
            &_ => panic!("Invalid crane protocol '{}' specified. Please specify CraneMover9000 or CraneMover9001", args[1])
        };
        let filename = &args[2].to_string();
        return (load_file_to_line_vector(filename,file_content_goes_here), protocol_goes_here) ;
    }
}

fn main() {
    let mut file_content = String::new();
    //let mut crane_protocol = 0;
    //let mut file_content_per_line;
    let result = prepare_for_parsing( &mut file_content); 
    let mut file_content_per_line = result.0;
    let crane_protocol=result.1;

    // Build a vector of vectors. I.e. a vector of the stacks.
    //
    let mut stacks : Vec<Vec<char>> = Vec::<Vec<char>>::new();


    //
    // Two parts below
    // 
    // Part one, load the stack data
    // Part two, process the move instructions
    // 
    // Both use the same iterator, handed over.
    // The first part "break" conveniently
    // letting the move-part continue until the
    // end of file.


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

    let mut sit = file_content_per_line.next();
 
    while sit != None {
        let s = sit.unwrap();
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
             
                // Read the actual content of the box
                let data = charit.next().unwrap();

                // Discard the closing bracket
                charit.next();

                // Check to make sure we have enough stack-data.
                while stacks.len() < stackindex+1 {
                    // Yet another stack, lets create space for it.
                    stacks.push(vec![]);
                    println!("Adding another stack, len is now {}", stacks.len());
                }
    
                // Final step, store the information
                println!("Pushing {} onto stack {}", data,stackindex);
                stacks[stackindex].insert(0,data);
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

            } else if nextchar == ' ' { // Emty slot, or the "axis maker", just read three characteres
                // rustc bug? If I use charit.skip(3) here instead
                // I get a compiler error on the statement 
                //     let data = charit.next().unwrap();
                // above.
                charit.next(); // This could possibly be a digit indicating the stack index, if its the "axis marker"
                charit.next(); 
                charit.next();
                stackindex+=1;
                nextchar_container = charit.next();
            }   
        }
        sit = file_content_per_line.next();
    }

    for st in &stacks {
        print!("Stack ");
        for ch in st {
            print!("{}", ch);
        }
        println!("\n");
    }



    //
    //
    // Part two the moving
    //

    sit = file_content_per_line.next();
    while sit != None {
        let s = sit.unwrap();
        
        // Read the instructions 
        // They look like this:
        // move 1 from 2 to 1
        let instructions : Vec<&str> = s.split(' ').collect();
        let count = instructions[1].parse::<usize>().unwrap();
        let from  = instructions[3].parse::<usize>().unwrap()-1;
        let to    = instructions[5].parse::<usize>().unwrap()-1;

        if crane_protocol == 1 {
            // CraneMover9000, one box at a time
            for n in 0..count{
                println!("Moving from {} to {} (iteration {})",from+1, to+1, n);
                let val = stacks[from].pop();
                stacks[to].push(val.unwrap());
            }  
        } else {
            // CraneMover9001, many boxes at a time
            println!("Moving {} boxes from {} to {} ",count, from+1, to+1);
            let last_to_take = stacks[from].len()-count;
            let mut u: Vec<_> = stacks[from].drain(last_to_take..).collect();

            stacks[to].append(&mut u);


        }


        sit = file_content_per_line.next();
    }

    for st in &stacks {
        print!("Stack after move ");
        for ch in st {
            print!("{}", ch);
        }
        print!("\n");
    }

    for st in &stacks {
        print!("{}",st[st.len()-1]);
    }
    println!("");

} 
