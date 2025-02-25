// consumer.rs
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // Lock standard input for safe reading
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => println!("Received: {}", text),
            Err(e) => eprintln!("Error reading input: {}", e),
        }
    }
}
