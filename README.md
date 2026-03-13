# Multithreaded Log Analyzer in Rust

## Overview

This project implements a highly efficient multithreaded log analysis tool written in Rust. The application is designed to ingest and parse large log files rapidly by distributing the computational workload across multiple worker threads. It demonstrates how to utilize safe concurrency patterns to process large text datasets without sacrificing performance or memory safety.

## Core Problem

Distributed systems, web servers, and application microservices generate massive volumes of diagnostic data. Parsing and analyzing these records sequentially often becomes a performance bottleneck. To address this challenge, the application processes the data concurrently, significantly reducing execution time. The project specifically tackles the complexities of sharing memory safely and dividing workloads evenly in a concurrent environment.

## Output
<img width="948" height="405" alt="Screenshot 2026-03-13 at 4 21 26 PM" src="https://github.com/user-attachments/assets/7fd97060-6229-4016-b78b-f64b6fe2510a" />

## Execution Workflow

The application executes through a defined sequence of operations:

* Data Generation: A companion utility generates a substantial synthetic dataset containing random diagnostic events logged as INFO, WARN, and ERROR.
* Memory Ingestion: The application reads the entirety of the log file into memory to minimize disk input and output operations during the analysis phase.
* Workload Preparation: The text data is parsed into individual strings and collected into a vector. This vector is then wrapped in an atomic reference counter to allow safe, shared access across multiple threads without duplication.
* Workload Distribution: The program calculates precise index boundaries to divide the total number of lines into equal segments for each designated worker thread.
* Parallel Computation: Each individual worker thread borrows a reference to the shared vector and processes its designated segment. The thread extracts the severity level from each line and increments its local statistics counter.
* Result Aggregation: Upon completion of all parallel computations, the main thread gathers the individual statistical structures from each worker and computes the global aggregation.

## Technology Advantages

Rust is specifically chosen for this application because of its strict compiler guarantees.

* Ownership and Borrowing: The compiler completely prevents data races by ensuring that shared string data is immutable across execution threads.
* Concurrency Primitives: By using thread joining mechanisms, the application safely extracts local results from independent tasks without requiring complex mutex locking mechanisms over global state.
* Shared Memory Efficiency: Distributing index ranges allows multiple threads to read from a single shared data source simultaneously, maximizing resource utilization.

## Usage Instructions

Ensure you have Rust and Cargo integrated into your system. Navigate to the project directory and invoke the standard build and execute command:

```text
cargo run
```

Upon successful execution, the console will output the segment allocations for each thread and precisely display the aggregated frequency of all parsed diagnostic levels.
