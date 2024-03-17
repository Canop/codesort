#[macro_export]
macro_rules! tprint {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        eprintln!($($arg)*);
    }
}

#[macro_export]
macro_rules! tprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        eprintln!($($arg)*);
    }
}
