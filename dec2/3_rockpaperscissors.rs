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

        let mut content: String = String::new();
        match fh.read_to_string(&mut content) {
            Err(reason) => panic!("Could not read {}: {}", filename, reason),
            Ok(x) => x
        };

        let split = content.split("\n");

        // A X Rock
        // B Y Paper
        // C Z Scissors

        let mut totalscore = 0;

        for s in split {
            let plays:Vec<&str> = s.split(" ").collect();
            let theyplay = plays[0];
            let weplay = plays[1];
            let score = match (theyplay, weplay) {
                ("A","X") => 1+3,//draw,
                ("A","Y") => 2+6,//win
                ("A","Z") => 3+0,//loss

                ("B","X") => 1+0,//loss
                ("B","Y") => 2+3,//draw,
                ("B","Z") => 3+6,//win

                ("C","X") => 1+6,//win
                ("C","Y") => 2+0,//loss
                ("C","Z") => 3+3,//draw,
                (_,_) => -10000000
            };
            totalscore += score;

        }
        println!("Total score is {}",totalscore);
    }
    
}
