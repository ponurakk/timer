use colored::Colorize;
use log::log_enabled;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub use timer_macro::{fn_timer, timer};

pub mod macros;

#[derive(Default)]
pub struct Timer {
    timers: HashMap<String, Instant>,
    avg: HashMap<String, (Vec<(Instant, Option<Instant>)>, u32)>,
    bottleneck: u64,
}

fn log(text: String) {
    if log_enabled!(log::Level::Info) {
        eprintln!("{}", text);
    }
}

impl Timer {
    pub fn new(bottleneck: Option<u64>) -> Self {
        Self {
            timers: HashMap::new(),
            avg: HashMap::new(),
            bottleneck: bottleneck.unwrap_or(700),
        }
    }

    pub fn start(&mut self, name: String) {
        self.timers.insert(name, Instant::now());
    }

    pub fn finish(&mut self, name: String, module_path: &str) {
        let time = self.timers.remove(&name);
        if let Some(duration) = time {
            let duration = if duration.elapsed() >= Duration::from_millis(self.bottleneck) {
                format!("[{}]", fmt(duration.elapsed())).red()
            } else {
                format!("[{}]", fmt(duration.elapsed())).blue()
            };
            log(format!("{} ({}) {}", duration, module_path.green(), name));
        }
    }

    pub fn start_avg(&mut self, name: String) {
        self.avg
            .entry(name)
            .and_modify(|v| {
                if let Some(last) = v.0.last_mut() {
                    last.1 = Some(Instant::now())
                }
                v.1 += 1;
            })
            .or_insert((vec![(Instant::now(), None)], 1));
    }

    pub fn tick_avg(&mut self, name: String) {
        self.avg.entry(name).and_modify(|v| {
            v.0.push((Instant::now(), None));
        });
    }

    pub fn finish_avg(&mut self, name: String, module_path: &str) {
        if let Some(avg) = self.avg.remove(&name) {
            let times = avg.1;
            let mut durations: Vec<Duration> = Vec::new();
            avg.0.iter().for_each(|item| {
                if let Some(finish) = item.1 {
                    durations.push(finish.duration_since(item.0));
                } else {
                    durations.push(Instant::now().duration_since(item.0))
                }
            });
            if !durations.is_empty() {
                let total_duration: Duration = durations.iter().sum();
                let average_duration = total_duration / durations.len() as u32;
                log(format!(
                    "{}/{} ({}) {}",
                    format!("{{{}}}", fmt(average_duration)).blue().bold(),
                    times,
                    module_path.green(),
                    name
                ));
            } else {
                log(format!(
                    "{}/{} ({}) {}",
                    format!("{{{}}}", "0.0ns").blue().bold(),
                    times,
                    module_path.green(),
                    name
                ));
            }
        }
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

    let time = major as f64 + (minor - major * 1000) as f64 / 1000.0;
    format!("{:.2}{}", time, t)
}
