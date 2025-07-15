use std::fs::{self, File};
use std::env;
use std::io::{self, BufReader, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Command requires an input file to process.");
        return;
    } else if args.len() < 3 {
        println!("Command requires an output file to process.");
        return;
    }
    
    let Ok(input) = read_lines(&args[1]) else { todo!() };
    let mut output = File::create(&args[2]);
}

fn read_lines<PathRef: AsRef<Path>>(filepath: PathRef) 
    -> io::Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines());
}
