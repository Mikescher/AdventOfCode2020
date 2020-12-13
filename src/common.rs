use std::sync::atomic::AtomicBool;

pub trait AdventOfCodeDay {
    fn task_1(&self) -> String;
    fn task_2(&self) -> String;
}

pub static PRINT_VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn is_verbose() -> bool {
    PRINT_VERBOSE.load(std::sync::atomic::Ordering::Relaxed)
}

macro_rules! verboseln {
    ($($arg:tt)*) => {
        if crate::common::is_verbose() {
            println!($($arg)*);
        }
    }
}

macro_rules! verbosedbg {
    ($val:expr) => {
        if crate::common::is_verbose() {
            dbg!($val);
        }
    };
    ($($val:expr),+ $(,)?) => {
        if crate::common::is_verbose() {
            dbg!($($val),+);
        }
    };
}

macro_rules! is_verbose {
    ($($arg:tt)*) => {
        crate::common::is_verbose()
    }
}