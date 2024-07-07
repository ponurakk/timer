#[macro_use]
extern crate timer;

use env_logger::{Builder, Env};
use std::collections::HashMap;

#[timer]
fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut hashmap: HashMap<u16, String> = HashMap::new();

    // You can time total execution time for loop
    start!(timer, "Total Loop");
    let mut _iters = 0;
    for i in 0..u16::MAX {
        // Or average per iteration
        start_avg!(timer, "Per iteration");
        hashmap.insert(i, String::from("Hello World"));

        // Mark finish of check
        tick_avg!(timer, "Per iteration");

        // This is not counted to average
        _iters += 1;
    }

    // Finish and print results
    finish_avg!(timer, "Per iteration");
    finish!(timer, "Total Loop");
}
