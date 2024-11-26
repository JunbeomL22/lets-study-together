# Trading System Development Prerequisites

## Table of Contents
1. [Working with PCAP Files](#1-working-with-pcap-files)
2. [Benchmarking Using Criterion](#2-benchmarking-using-criterion)
3. [Understanding Memory Alignment](#3-understanding-memory-alignment)
4. [Working with Timestamps](#4-working-with-timestamps)
5. [Thread Communication](#5-thread-communication)

This repository is designed to help you learn the fundamental knowledge needed to participate in trading system development. Each participant has their own crate within this workspace.

Before making a pull request, please run `cargo check`, `cargo test`, `cargo bench`, and `cargo clippy` from the root directory. These commands will run all tests and benchmarks across the entire workspace. We do this to help you become familiar with writing Cargo.toml configurations for workspace management.

If you want to test your individual work, you can run cargo commands within your own crate's directory. We've provided a 'common' crate as an example to help you understand how to structure your own crate.

Let's look at what you'll be learning:

## 1. Working with PCAP Files
We'll start by extracting basic mid-price information from PCAP files. You'll create a structure like this:
```Rust
pub struct MidPrice {
   values: Vec<f64> 
}
```

You'll implement a push method and iterate through a PCAP file to populate the values. Since PCAP iteration is quite heavy, you'll save the MidPrice structure to a JSON file (recommend using the serde crate). Then, you'll read the JSON file back and verify it matches the original structure. Add this verification process to your unit tests and run `cargo test` to check everything works.

## 2. Benchmarking Using Criterion
Let's explore performance differences between numeric types. You'll benchmark arithmetic operations with f32, f64, i32, i64 using the criterion crate. Test both single variables and vectors of different sizes. Think about why you're seeing these results - the answers might surprise you!

## 3. Understanding Memory Alignment
Create a structure like this:
```Rust
pub struct Dummy {
    pub val: f64,
    pad: u8
}
```

First, investigate how much memory this structure actually uses. Then implement std::ops::Mul for it and compare multiplication times between Vec<f64> and Vec<Dummy>. The performance difference might be significant - can you figure out why?

## 4. Working with Timestamps
Time is crucial in trading systems. Compare how long it takes to get the current time using different approaches: chrono, time, and std. Then try using quanta for high-precision timing and think about how to create Unix nano timestamps with it.

## 5. Thread Communication
Let's tackle a real-world scenario. Create a --bin crate in your example folder. Consider this simple task:
```Rust
for i in 0..1000 {
    println!("{}", i);
}
```

While this might seem simple, it represents something crucial in trading systems - logging. We need to record orders, cancellations, balances, and profits somewhere, whether to files or console. This is what we call logging.

Good logging is essential for system stability, but it shouldn't slow down our main operations. That's why we'll split this into two threads: one for main operations and one for logging.

Create two threads - a worker thread and a logging thread. Have the worker thread send numbers 0 to 999 to the logging thread, which will handle the println! operations. This changes the bottleneck from println! time to thread communication time. Measure how long it takes to send all these numbers between threads.

This exercise demonstrates how we can prevent intensive I/O operations from blocking our main trading logic - a crucial concept in high-performance trading systems.

Feel free to ask questions and discuss your findings. The goal is to understand these concepts deeply, as they're fundamental to building efficient trading systems.