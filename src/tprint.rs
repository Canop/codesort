/// `eprint!` but only if the explain feature is active
#[macro_export]
macro_rules! tprint {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        eprintln!($($arg)*);
    }
}

/// `eprintln!` but only if the explain feature is active
#[macro_export]
macro_rules! tprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        eprintln!($($arg)*);
    }
}
