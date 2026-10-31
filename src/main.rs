use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

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

                #[allow(unused_variables)]
                let command = match command_str.as_str() {
                    "start" => StopWatch::Start,
                    "stop" => StopWatch::Stop,
                    "lap" => StopWatch::Lap,
                    "exit" => StopWatch::Exit,
                    _ => StopWatch::Invalid,
                };

                match command {
                    StopWatch::Start => {
                        println!("StopWatch started");
                        let start_time = Instant::now();

                        loop {
                            let elapsed = start_time.elapsed();

                            print!(
                                "\rElapsed: {:02}:{:02}.{:03}",
                                elapsed.as_secs() / 60,
                                elapsed.as_secs() % 60,
                                elapsed.subsec_millis()
                            );

                            io::stdout().flush().unwrap();
                            thread::sleep(Duration::from_millis(50));
                        }
                    }

                    StopWatch::Stop => {
                        println!("StopWatch stopped");
                        break;
                    }

                    StopWatch::Lap => {
                        println!("Lap time");
                        break;
                    }

                    StopWatch::Exit => {
                        println!("Exiting StopWatch");
                        break;
                    }

                    StopWatch::Invalid => {
                        println!("Invalid command");
                        break;
                    }
                }
            }

            Err(e) => {
                todo!("Failed to read line: {}", e);
            }
        }
    }
}
