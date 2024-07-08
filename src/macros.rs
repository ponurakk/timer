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

#[macro_export]
macro_rules! start_avg {
    ($timer:expr, $name:expr) => {
        $timer.start_avg($name);
    };
}

#[macro_export]
macro_rules! tick_avg {
    ($timer:expr, $name:expr) => {
        $timer.tick_avg($name);
    };
}

#[macro_export]
macro_rules! finish_avg {
    ($timer:expr, $name:expr) => {
        $timer.finish_avg($name, module_path!());
    };
}
