//! A simple macro to time and log the execution time of a function.

use colored::Colorize;
use log::log_enabled;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub use concat_string::concat_string;
pub use timer_macro::{fn_timer, timer};
pub mod macros;

type TimeSpan = (Instant, Option<Instant>);

/// A simple timer to time and log the execution time of a function.
#[derive(Default)]
pub struct Timer<'a> {
    timers: HashMap<&'a str, Instant>,
    avg: HashMap<&'a str, (Vec<TimeSpan>, u32)>,
    bottleneck: u64,
}

fn log(text: &str) {
    if log_enabled!(log::Level::Debug) {
        eprintln!("{text}");
    }
}

impl<'a> Timer<'a> {
    /// Create a new timer
    #[must_use]
    pub fn new(bottleneck: Option<u64>) -> Self {
        Self {
            timers: HashMap::new(),
            avg: HashMap::new(),
            bottleneck: bottleneck.unwrap_or(700),
        }
    }

    /// Start a timer
    pub fn start(&mut self, name: &'a str) {
        self.timers.insert(name, Instant::now());
    }

    /// Finish a timer
    pub fn finish(&mut self, name: &'a str, module_path: &str) {
        let Some(start_time) = self.timers.remove(&name) else {
            return;
        };

        let elapsed = start_time.elapsed();
        let duration = if elapsed >= Duration::from_millis(self.bottleneck) {
            format!("[{}]", fmt(elapsed)).red()
        } else {
            format!("[{}]", fmt(elapsed)).blue()
        };

        log(&format!("{duration} ({}) {name}", module_path.green()));
    }

    /// Start a timer for loop averaging
    pub fn start_avg(&mut self, name: &'a str) {
        let now = Instant::now();

        self.avg
            .entry(name)
            .and_modify(|(items, times)| {
                if let Some((_, end)) = items.last_mut() {
                    *end = Some(now);
                }

                *times += 1;
            })
            .or_insert((vec![(now, None)], 1));
    }

    /// Finish a timer for loop averaging
    pub fn tick_avg(&mut self, name: &'a str) {
        if let Some((items, _)) = self.avg.get_mut(&name) {
            items.push((Instant::now(), None));
        }
    }

    /// Finish a timer for loop averaging
    pub fn finish_avg(&mut self, name: &'a str, module_path: &str) {
        let Some((items, times)) = self.avg.remove(&name) else {
            return;
        };

        let mut total_duration = Duration::new(0, 0);

        for (start, end) in items {
            total_duration += if let Some(finish) = end {
                finish.duration_since(start)
            } else {
                Instant::now().duration_since(start)
            };
        }

        let avg_duration = if times > 0 {
            total_duration / times
        } else {
            Duration::new(0, 0)
        };

        log(&format!(
            "{}/{times} ({}) {name}",
            format!("{{{}}}", fmt(avg_duration)).blue().bold(),
            module_path.green(),
        ));
    }
}

fn fmt(elapsed: Duration) -> String {
    let secs = u128::from(elapsed.as_secs());
    let millis = elapsed.as_millis();
    let micros = elapsed.as_micros();
    let nanos = elapsed.as_nanos();

    let (major, minor, t) = if secs > 0 {
        (secs, millis, "s")
    } else if millis > 0 {
        (millis, micros, "ms")
    } else if micros > 0 {
        (micros, nanos, "μs")
    } else {
        (nanos, nanos * 1000, "ns")
    };

    #[allow(clippy::cast_precision_loss)]
    let time = major as f64 + (minor - major * 1000) as f64 / 1000.0;
    format!("{time:.2}{t}")
}
