use std::io::{self, Write};
#[allow(unused_imports)]
use std::time::Instant;

#[derive(Debug)]
enum StopWatch {
    Start,
    Stop,
    Lap,
    Exit,
    Invalid,
}

fn main() {
    println!("***** Welcome to StopWatch *****");
    println!("Commands: Start, Stop, Lap, Exit");

    //Loop to continuously get input from the user
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command_str = input.trim().to_lowercase();
            }
            Err(e) => {
                todo!("Failed to read line: {}", e);
            }
        }
    }
}
