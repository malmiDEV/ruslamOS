#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::sys::shell::write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub mod drivers;
pub mod shell;
