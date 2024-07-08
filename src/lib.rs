use colored::Colorize;
use log::log_enabled;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub use concat_string::concat_string;
pub use timer_macro::{fn_timer, timer};
pub mod macros;

#[derive(Default)]
pub struct Timer<'a> {
    timers: HashMap<&'a str, Instant>,
    avg: HashMap<&'a str, (Vec<(Instant, Option<Instant>)>, u32)>,
    bottleneck: u64,
}

fn log(text: String) {
    if log_enabled!(log::Level::Info) {
        eprintln!("{}", text);
    }
}

impl<'a> Timer<'a> {
    pub fn new(bottleneck: Option<u64>) -> Self {
        Self {
            timers: HashMap::new(),
            avg: HashMap::new(),
            bottleneck: bottleneck.unwrap_or(700),
        }
    }

    pub fn start(&mut self, name: &'a str) {
        self.timers.insert(name, Instant::now());
    }

    pub fn finish(&mut self, name: &'a str, module_path: &str) {
        if let Some(start_time) = self.timers.remove(&name) {
            let elapsed = start_time.elapsed();
            let duration = if elapsed >= Duration::from_millis(self.bottleneck) {
                format!("[{}]", fmt(elapsed)).red()
            } else {
                format!("[{}]", fmt(elapsed)).blue()
            };
            log(format!("{} ({}) {}", duration, module_path.green(), name));
        }
    }

    pub fn start_avg(&mut self, name: &'a str) {
        let now = Instant::now();

        self.avg
            .entry(name)
            .and_modify(|v| {
                if let Some(last) = v.0.last_mut() {
                    last.1 = Some(now);
                }
                v.1 += 1;
            })
            .or_insert((vec![(now, None)], 1));
    }

    pub fn tick_avg(&mut self, name: &'a str) {
        if let Some(v) = self.avg.get_mut(&name) {
            v.0.push((Instant::now(), None));
        }
    }

    pub fn finish_avg(&mut self, name: &'a str, module_path: &str) {
        if let Some(avg) = self.avg.remove(&name) {
            let times = avg.1;
            let mut total_duration = Duration::new(0, 0);

            for item in avg.0 {
                total_duration += if let Some(finish) = item.1 {
                    finish.duration_since(item.0)
                } else {
                    Instant::now().duration_since(item.0)
                };
            }

            let avg_duration = if times > 0 {
                total_duration / times as u32
            } else {
                Duration::new(0, 0)
            };

            log(format!(
                "{}/{} ({}) {}",
                format!("{{{}}}", fmt(avg_duration)).blue().bold(),
                times,
                module_path.green(),
                name
            ));
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
