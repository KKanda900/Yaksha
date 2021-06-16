/*
 * Yaksha Shell Program:
 * 
 * Description: 
 * A shell program that has linux termainal functions such as ls, cd, clear, ... and much much more.
 */

use whoami::username;
use std::io;
use std::io::stdin;
use std::io::prelude::*;
use std::env::current_dir;
use std::fmt::Display;
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

fn process_input(input: String, curr_directory: String) {
    if input == "ls" {
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
        }
    }
}

// starts the main termainal program
fn main(){
    curr_directory = current_dir().unwrap().display();
    init_shell();
    loop {
        println!("Current Directory: {}>", curr_directory);
        let input = get_input(); // string input
        process_input(input, curr_directory);
        break;
    }
}
