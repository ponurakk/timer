#![allow(missing_docs)]

#[macro_use]
extern crate timer;

use env_logger::{Builder, Env};

// Limit your time wanted to execute in ms
#[timer(bottleneck = 1)]
fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    start!(timer, "Total Loop");
    for _ in 0..10_000_000 {}

    // Output should be red
    // indicating that the limit was exceded
    finish!(timer, "Total Loop");

    #[cfg(not(debug_assertions))]
    println!("You should try running it in debug mode as release is too optimized ¯\\_(ツ)_/¯");
}
