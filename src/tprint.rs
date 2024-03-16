macro_rules! tprint {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        println!($($arg)*);
    }
}
macro_rules! tprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        println!($($arg)*);
    }
}

pub(crate) use {
    tprint,
    tprintln,
};
