use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::cell::RefCell;

thread_local! { // Each thread has its own copy.
    static HAD_ERROR: RefCell<bool> = RefCell::new(false); // we wrap the bool inside a RefCell, which allows us to mutate it at runtime without mut.
}

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
        run_prompt();
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
    println!("Type in your input:");
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    loop {
        print!("> "); 
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        input.clear(); // Clear previous input
        if handle.read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            break;
        }

        let trimmed = input.trim(); // Remove trailing newlines
        if trimmed.is_empty() { // This properly checks for empty input
            break;
        }

        run(&trimmed.to_string());
    }
}

fn run(source: &String) { // Not sure how to define the return type yet
    for token in source.chars() {
        println!("{}", token);
    }
}

fn error(line: u32, message: &String) {

}

fn report(line: u32, loc: &String, message: &String) {
    println!("[line {}] Error {}: {}", line, loc, message);
    
    // Set the global hadError flag; we can’t access a thread_local! variable directly. Instead, we use .with():
    HAD_ERROR.with(|had_error| *had_error.borrow_mut() = true); // .borrow_mut() gets a mutable reference to the bool inside RefCell.
}