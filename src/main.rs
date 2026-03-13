use std::fs;

mod log_generator;

use log_generator::log_generator;

fn main() {
    log_generator().unwrap();
    // getting the logs in memory
    let logs = fs::read_to_string("logs.txt").expect("failed to read log files");
    println!("Loaded {} bytes of logs", logs.len());

    // at this point -> logs.txt ------> String in memory

    let lines: Vec<&str> = logs.lines().collect();
    println!("Loaded {} lines", lines.len());

    // &str → "INFO Server started"
    //   ├─ &str → "INFO User logged in"
    //   ├─ &str → "ERROR Database connection failed"
    //   └─ &str → "WARN Disk almost full"

    // no new strings are being created each new line borrows from the original string
    // this is called zero copy parsing

    // copying every line to the memory would be expensive as my ex
    println!("Total log lines: {}", lines.len());
}
