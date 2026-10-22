use std::time::Instant;

enum StopWatchCommands {
    Start,
    Stop,
    Lap,
    Exit,
}

fn main() {
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}
