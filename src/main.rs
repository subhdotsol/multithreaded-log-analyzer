use std::fs;
use std::sync::Arc;
use std::thread;

mod log_generator;

use log_generator::log_generator;

struct ThreadStats {
    info: usize,
    warn: usize,
    error: usize,
}

fn main() {
    // generate synthetic logs
    log_generator().unwrap();

    // Read entire file into memory
    // logs owns the string data
    // load file into memory
    let logs = fs::read_to_string("logs.txt").expect("failed to read log file");

    println!("Loaded {} bytes of logs", logs.len());

    // logs.txt -----> String stored in memory

    // logs.lines() creates &str slices that BORROW from logs
    // IMPORTANT: these are NOT new strings
    // they are references pointing inside the `logs` String
    //
    // zero-copy version (won't work with threads!):
    // let lines: Vec<&str> = logs.lines().collect();
    //
    // fix : we need owned Strings so threads can take ownership
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

    // share lines safely between threads
    // Arc<Vec<String>> can be shared across threads since String is 'static
    let lines = Arc::new(lines);

    // parsing the lines in different threads
    let num_threads = 5;

    let chunk_size = lines.len() / num_threads;

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let lines = Arc::clone(&lines);

        let start = i * chunk_size;

        let end = if i == num_threads - 1 {
            lines.len()
        } else {
            (i + 1) * chunk_size
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
        //
        // fix : instead of copying chunks, we use Arc to share
        // the lines and access them by index range inside the thread

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
        // fix : use Arc<Vec<&str>> and access by index range
        // the Arc keeps the Vec alive as long as any thread holds a clone
        let handle = thread::spawn(move || {
            println!("Thread {} processing lines {} to {}", i, start, end);

            let mut info = 0;
            let mut warn = 0;
            let mut error = 0;

            for line in &lines[start..end] {
                // line is &str borrowed from logs

                let mut parts = line.split_whitespace();

                // parts.nth(1) skips the first item (timestamp)
                // and gets the second item (log level)
                let level = parts.nth(1).unwrap_or("");

                match level {
                    "INFO" => info += 1,
                    "WARN" => warn += 1,
                    "ERROR" => error += 1,
                    _ => {}
                }
            }

            ThreadStats { info, warn, error }
        });

        handles.push(handle);
    }

    // merge results
    let mut total_info = 0;
    let mut total_warn = 0;
    let mut total_error = 0;

    for handle in handles {
        let stats = handle.join().unwrap();

        total_info += stats.info;
        total_warn += stats.warn;
        total_error += stats.error;
    }

    println!("\nLog Analysis Complete\n");

    println!("INFO: {}", total_info);
    println!("WARN: {}", total_warn);
    println!("ERROR: {}", total_error);
}
