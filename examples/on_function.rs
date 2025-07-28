#![allow(missing_docs)]

#[macro_use]
extern crate timer;

use env_logger::{Builder, Env};

// You can time entire function
#[fn_timer(bottleneck = 300)]
fn my_fn(text: String) -> String {
    // Super unoptimal and not needed but shows it is working still with return value
    let out: Vec<String> = text.chars().map(|v| v.to_uppercase().to_string()).collect();
    out.iter().cloned().collect::<String>()
}

// And still have access to individual timers
#[fn_timer]
fn iter_fn() {
    for _ in 0..u16::MAX {
        start_avg!(timer, "Inside iter");
        // Some work
        tick_avg!(timer, "Inside iter");
    }
    finish_avg!(timer, "Inside iter");
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let text = my_fn("Hello World".to_string());
    println!("{text}");

    iter_fn();
}
