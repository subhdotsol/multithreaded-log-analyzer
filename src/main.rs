use std::fs;
use std::thread;

mod log_generator;

use log_generator::log_generator;

fn main() {
    log_generator().unwrap();

    // Read entire file into memory
    // logs owns the string data
    let logs = fs::read_to_string("logs.txt").expect("failed to read log files");

    println!("Loaded {} bytes of logs", logs.len());

    // logs.txt -----> String stored in memory

    // logs.lines() creates &str slices that BORROW from logs
    // IMPORTANT: these are NOT new strings
    // they are references pointing inside the `logs` String
    // let lines: Vec<&str> = logs.lines().collect();
    // fix :
    let lines: Vec<String> = logs.lines().map(|line| line.to_string()).collect();

    println!("Loaded {} lines", lines.len());

    // Memory layout looks like this:
    //
    // logs (String)
    // ├── "INFO Server started"
    // ├── "INFO User logged in"
    // ├── "ERROR Database connection failed"
    // └── "WARN Disk almost full"
    //
    // lines (Vec<&str>)
    // ├── &str -> points inside logs
    // ├── &str -> points inside logs
    // ├── &str -> points inside logs
    // └── &str -> points inside logs
    //
    // so `lines` depends on `logs` being alive

    // zero-copy parsing
    // we avoided allocating new Strings
    println!("Total log lines: {}", lines.len());

    // parsing the lines in different threads
    let num_threads = 5;

    let chunck_size = lines.len() / num_threads;

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let start = i * chunck_size;

        let end = if i == num_threads - 1 {
            lines.len()
        } else {
            (i + 1) * chunck_size
        };

        // ERROR #2 HAPPENS HERE
        //
        // lines[start..end] -> slice of Vec<&str>
        // .to_vec() -> creates a NEW Vec<&str>
        //
        // BUT you immediately take a reference to it
        //
        // temporary Vec<&str> created here
        //        ↓
        // &lines[start..end].to_vec()
        //
        // this means:
        //
        // chunk -> reference to temporary vector
        //
        // after this line finishes, the temporary vector is DROPPED
        // leaving `chunk` pointing to invalid memory
        //
        // Rust prevents this with error E0716
        // let chunk = &lines[start..end].to_vec();
        // fix :
        let chunk = lines[start..end].to_vec();

        // ERROR #1 HAPPENS HERE
        //
        // thread::spawn requires the closure to be `'static`
        //
        // meaning:
        // everything captured by the thread must live for the
        // entire lifetime of the thread
        //
        // but `chunk` contains &str references
        //
        // those &str references point into `logs`
        //
        // logs is a local variable inside main
        // so it will be DROPPED when main ends
        //
        // Rust cannot guarantee that the thread will finish
        // before `logs` is dropped
        //
        // therefore Rust rejects it with:
        //
        // error[E0597]: logs does not live long enough
        //
        let handle = thread::spawn(move || {
            for line in chunk {
                // line is &str borrowed from logs

                let mut parts = line.split_whitespace();

                let level = parts.next().unwrap_or("");

                match level {
                    "INFO" => println!("INFO log found"),

                    "WARN" => println!("WARN log found"),

                    "ERROR" => println!("ERROR log found"),

                    _ => {}
                }
            }
        });

        handles.push(handle);
    }
}
