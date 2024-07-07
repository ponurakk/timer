#[macro_use]
extern crate timer;

use env_logger::{Builder, Env};
use std::collections::HashMap;

#[fn_timer]
fn my_fn(text: String) -> String {
    // Super unoptimal and not needed but shows it is working still with return value
    let out: Vec<String> = text.chars().map(|v| v.to_uppercase().to_string()).collect();
    out.iter().cloned().collect::<String>()
}

#[fn_timer]
fn iter_fn() {
    for _ in 0..u16::MAX {}
}

#[timer]
fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut hashmap: HashMap<u16, String> = HashMap::new();

    start!(timer, "Total Loop");
    for i in 0..u16::MAX {
        start_avg!(timer, "Per iteration");
        hashmap.insert(i, String::from("Hello World"));
        tick_avg!(timer, "Per iteration");
    }
    finish_avg!(timer, "Per iteration");
    finish!(timer, "Total Loop");

    let text = my_fn("Hello World".to_string());
    println!("{text}");

    iter_fn();
}
