/*
 * Yaksha Shell Program:
 * 
 * Description: 
 * A shell program that has linux terminal functions such as ls, cd, clear, ... and much much more.
 */

use whoami::username;
use std::io;
use std::io::prelude::*;
use std::env::current_dir;
use walkdir::WalkDir;
use std::vec::Vec;
use std::path::PathBuf;

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
    let line1 = "";
    let line2 = iterator.next().unwrap().unwrap();
    return line2;
}

// process input
// take the full input then process the full first word -> command
// take the second full input -> args for the command
fn process_input(input: String, curr_directory: String) -> String{
    
    // parse the input => split the input into command and arguments
    let split_input = input.split(" ");
    let mut input_arr = Vec::new();
    for token in split_input {
        input_arr.push(token);
    }

    // get the commands and arguments
    let command = input_arr[0];
    let mut args = Vec::new();
    for inputs in input_arr {
        args.push(inputs);
    }

    // handle single commands
    if command == "ls" {
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
        }
    }

    if command == "exit" {
        return "exit".to_string();
    }

    if command == "cls" {
        clear_screen();
    }

    // handle the cd command -> change directory
    // should only be one argument (the directory that you want to go to)
    if command == "cd" {
        let new_directory = curr_directory + "\\" + args[1];
        return new_directory.to_string();
    }

    return "".to_string();

}


// starts the main termainal program
fn main() -> std::io::Result<()>{
    init_shell();
    let mut path = current_dir()?;
    loop {
        println!("{}>", path.display());
        let input = get_input(); // string input
        let return_output = process_input(input, path.display().to_string());
        if return_output == "exit" {
            break;
        }
    }
    Ok(())
}


