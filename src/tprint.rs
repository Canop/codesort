macro_rules! tprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "explain")]
        println!($($arg)*);
    }
}

pub(crate) use tprintln;
