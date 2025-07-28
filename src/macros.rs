//! Macros for the timer crate

/// Start a timer
#[macro_export]
macro_rules! start {
    ($timer:expr, $name:expr) => {
        $timer.start($name);
    };
    ($timer:expr, $($name:expr),+) => {
        let string = concat_string!($($name),+);
        $timer.start(&string);
    };
}

/// Finish a timer
#[macro_export]
macro_rules! finish {
    ($timer:expr, $name:expr) => {
        $timer.finish($name, module_path!());
    };
    ($timer:expr, $($name:expr),+) => {
        let string = concat_string!($($name),+);
        $timer.finish(&string, module_path!());
    };
}

/// Start a timer for loop averaging
#[macro_export]
macro_rules! start_avg {
    ($timer:expr, $name:expr) => {
        $timer.start_avg($name);
    };
}

/// Tick a timer for loop averaging
#[macro_export]
macro_rules! tick_avg {
    ($timer:expr, $name:expr) => {
        $timer.tick_avg($name);
    };
}

/// Finish a timer for loop averaging
#[macro_export]
macro_rules! finish_avg {
    ($timer:expr, $name:expr) => {
        $timer.finish_avg($name, module_path!());
    };
}
