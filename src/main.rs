use std::env;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

// This is a comment, and is ignored by the compiler.
// This is the main function.
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        println!("Usage: ./rlox [script]");
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error: {}", e); // 🔥 Now errors will be shown in the terminal!
        }
    } else {
        let _ = run_prompt();
    }
}

fn run_file(file_path: &String) -> io::Result<()>  { // io::Result<()>, explicitly indicating success or failure.
    println!("{}", file_path);
    let path = Path::new(file_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Ok(file) => file, // file now contains the File handle
        Err(e) => { // Old school error checking
            eprintln!("Error opening file {}: {}", display, e);
            return Err(e);
        }
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s)?; // If this fails, the error is returned to the caller; ? => Error propagate
    run(&s);
    Ok(())
}

fn run_prompt() {

}

fn run(source: &String) { // Not sure how to define the return type yet
    println!("Input file contains: {source}");
}