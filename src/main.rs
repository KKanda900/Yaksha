/*
 * Yaksha Shell Program:
 * 
 * Description: 
 * A shell program that has linux termainal functions such as ls, cd, clear, ... and much much more.
 */

use whoami::username;



fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

fn init_shell() {
    clear_screen();
    print!("\n\n\n\n******************************************");
    print!("\n\n\t\t**** Yaksha ****");
    println!("\n\n\t\t-Have Fun!-");
    println!("******************************************");
    print!("\n\n\nUSER is: @{}", username());
    print!("\n");
    clear_screen();
}

// starts the main termainal program
fn main(){
    init_shell();
}
