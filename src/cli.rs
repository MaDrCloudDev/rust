// How you might create CLI applications
use std::env::args;

pub fn run() {
    let args: Vec<String> = args().collect();
    let command = args[1].clone();
    let name = "Matt";
    let status = "100%";

    // println!("Command: {:?}", command)
    // cargo run hello, cargo run status

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Not a valid command");
    }
}