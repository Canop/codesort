use {
    lazy_regex::regex_captures,
    std::{
        num::{
            NonZeroUsize,
            ParseIntError,
        },
        str::FromStr,
    },
};

/// A 1-based line number, as used in most text editors
///
/// This is used for exchanging with the user, and for most
/// APIs. Disambiguation is done by using either `LineNumber`
/// or `LineIndex`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LineNumber {
    pub number: NonZeroUsize,
}

/// A range of 1-based line numbers, both ends included
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineNumberRange {
    pub start: LineNumber,
    pub end: LineNumber,
}

/// A 0-based line index
pub type LineIndex = usize;

impl FromStr for LineNumber {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse()?;
        Ok(LineNumber { number })
    }
}

impl FromStr for LineNumberRange {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_, start, end)) = regex_captures!(r"^(\d+)[^\d]+(\d+)$", s) else {
            return Err(format!("Invalid line number range: {}", s));
        };
        let start: LineNumber =
            start.parse().map_err(|e: ParseIntError| e.to_string())?;
        let end: LineNumber = end.parse().map_err(|e: ParseIntError| e.to_string())?;
        if start >= end {
            return Err(format!("Invalid range: {}", s));
        }
        Ok(LineNumberRange { start, end })
    }
}

impl LineNumber {
    pub fn new(number: usize) -> Option<Self> {
        NonZeroUsize::new(number).map(|number| LineNumber { number })
    }
    pub fn to_index(&self) -> LineIndex {
        self.number.get() - 1
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}
