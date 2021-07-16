/*
 * Yaksha Shell Program:
 * 
 * Description: 
 * A shell program that has linux termainal functions such as ls, cd, clear, ... and much much more.
 */

use whoami::username;
use std::io;
use std::io::prelude::*;
use std::env::current_dir;
use walkdir::WalkDir;

fn clear_screen(){
    print!("{}[2J", 27 as char);
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn init_shell() {
    println!("--IMPORTANT:----------------------------------");
    println!("Welcome to Yaksha :)");
    println!("----------------------------------------------");
    print!("\n\n\nUSER is: @{}", username());
    print!("\n");
    pause();
    clear_screen();
}

fn get_input() -> String{
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    return line2;
}

// process input
// take the full input then process the full first word -> command
// take the second full input -> args for the command
fn process_input(input: String, curr_directory: String) {
    
    // parse the input
    let split_input = input.split(" ");

    

    /* if input == "ls" {
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
        }
    } */
}

/* fn return_directory() -> std::io::Result<(), std::io::Error> {
    let path = current_dir()?;
    println!("The current directory is {}", path.display());
    return path;
} */

// starts the main termainal program
fn main() -> std::io::Result<()>{
    init_shell();
    let path = current_dir()?;
    loop {
        println!("The current directory is {}", path.display());
        let input = get_input(); // string input
        process_input(input, path.display().to_string());
        break;
    }
    Ok(())
}


