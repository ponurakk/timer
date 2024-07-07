#[macro_export]
macro_rules! start {
    ($timer:expr, $name:expr) => {
        $timer.start($name.to_string())
    };
}

#[macro_export]
macro_rules! finish {
    ($timer:expr, $name:expr) => {
        $timer.finish($name.to_string(), module_path!());
    };
}

#[macro_export]
macro_rules! start_avg {
    ($timer:expr, $name:expr) => {
        $timer.start_avg($name.to_string())
    };
}

#[macro_export]
macro_rules! tick_avg {
    ($timer:expr, $name:expr) => {
        $timer.tick_avg($name.to_string())
    };
}

#[macro_export]
macro_rules! finish_avg {
    ($timer:expr, $name:expr) => {
        $timer.finish_avg($name.to_string(), module_path!());
    };
}
