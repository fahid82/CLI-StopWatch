use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
enum StopWatch {
    Start,
    Stop,
    Lap,
    Exit,
}

impl StopWatch {
    fn parse(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "start" => Some(StopWatch::Start),
            "stop" => Some(StopWatch::Stop),
            "lap" => Some(StopWatch::Lap),
            "exit" => Some(StopWatch::Exit),
            _ => None,
        }
    }
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
            Ok(_) => match StopWatch::parse(&input) {
                Some(StopWatch::Start) => {
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

                Some(StopWatch::Stop) => {
                    println!("StopWatch stopped");
                }

                Some(StopWatch::Lap) => {
                    println!("Lap time");
                }

                Some(StopWatch::Exit) => {
                    println!("Exiting StopWatch");
                    break;
                }

                None => {
                    println!("Invalid command");
                }
            },

            Err(e) => {
                todo!("Failed to read line: {}", e);
            }
        }
    }
}
