#[macro_use]
extern crate timer;

use env_logger::{Builder, Env};

// Limit your time wanted to execute in ms
#[timer(bottleneck = 1)]
fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    start!(timer, "Total Loop");
    for _ in 0..1_000_000 {}

    // Output should be red
    // indicating that the limit was exceded
    finish!(timer, "Total Loop");
}
