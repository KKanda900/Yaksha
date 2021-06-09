use std::io::stdin;
use std::io::stdout;

/*
Notes on Print statements in Rust:
    - for strings: print!("STRING")
    - for any variable: print!("{}", variable)
 */
/* 
Example
fn main(){
    print!("Hello World");
    print!("{}",2+2);
} 
*/

/*  
Notes on Variables in Rust:
    - let -> Variable types -> don't need to specify the type
    - const -> Constant type -> need to specify the type
    - types/lengths:
        Length	Signed	Unsigned
        8-bit	  i8	  u8
        16-bit	  i16	  u16
        32-bit	  i32	  u32
        64-bit	  i64	  u64
        128-bit	  i128	  u128
        arch	  isize	  usize
 */
/* 
Example
fn main(){
    let vara = 10; // variable
    const varb: u8 = 12; // constants
    print!("This is a let variable {}\n", vara);
    print!("This is a const variable {}", varb);
} 
*/

/*
Notes on conditionals in Rust:
    - if {}... else {}
    - with variables: let something = if condition {this} else {that}
 */
 
/*  
Example:
fn main(){
     if 1+1==2 {
         println!("HELLO");
     } else {
         println!("HELLO WORLD");
     }
     let a = 10;
     let b = if a <= 5 {4} else {6};
     println!("{}",a);
     println!("{}", b);
 } 
 */

 /*
Notes on loops in Rust:
    - Work like regular loops in other languages with () while condition {}, for condition {}, ....
    - for infinite loop -> loop {}
*/

/* 
Example:
fn main() {
    let i = 100;
    for n in 0..i {
        println!("HELLO")
    }

    let mut i = 0;
    while i <= 100 {
        println!("HELLO");
        i = i+1;
    }

    loop {
        print!("Hello");
}
} */

/*
Notes on Threads in Rust:
    - thread::spawn(|| {
        function of thread
     }) -> spawns threads and does action
    - thread::sleep(Duration::from_millis(1)) -> current thread sleeps
    - handle.join().unwrap() -> waits for all the threads to generate
*/

use std::thread;
use std::time::Duration;

/* fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

} */

/* fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
} */

use std::process::Command;
use std::process::{Command, Stdio};

/*
Notes on Processes in Rust:
    - spawning a process
*/

/* fn main(){
    // stdout must be configured with `Stdio::piped` in order to use
    // `echo_child.stdout`
    let echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    // Note that `echo_child` is moved here, but we won't be needing
    // `echo_child` anymore
    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    let mut sed_child = Command::new("sed")
        .arg("s/tpyo/typo/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start sed process");

    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());
} */