use rand::RngExt;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn log_generator() -> Result<String, Box<dyn Error>> {
    let levels = ["INFO", "WARN", "Error"];

    let messages = [
        "Server started",
        "User logged in",
        "Request processed",
        "Database connection failed",
        "Timeout",
        "Disk almost full",
        "Authentication failed",
        "Memory usage high",
    ];

    let file = File::create("logs.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let mut rng = rand::rng();
    let total_logs = 1_000_000;

    for i in 0..total_logs {
        let level = levels[rng.random_range(0..levels.len())];
        let message = messages[rng.random_range(0..messages.len())];

        let log_line = format!(
            "2026-03-13T10:{:02}:{:02} {} {}\n",
            rng.random_range(0..60),
            rng.random_range(0..60),
            level,
            message
        );

        writer.write_all(log_line.as_bytes()).unwrap();
        if i % 100000 == 0 {
            println!("Generated {} logs", i);
        }
    }
    Ok("Logs generated successfully".to_string())
}
