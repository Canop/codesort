use crate::*;

/// An error in codesort
#[derive(thiserror::Error, Debug)]
pub enum CsError {
    #[error("You can't specify both --around and --range")]
    RangeAndAround,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Fmt error: {0}")]
    Fmt(#[from] std::fmt::Error), // only happens in debug

    #[error("No sortable range found around line {}", .0+1)]
    NoSortableRangeAround(LineIndex),

    #[error("Invalid range {}..{}", .start+1, .end+1)]
    InvalidRange { start: LineIndex, end: LineIndex },

    #[error("Provided range not sortable (lang: {0:?})")]
    RangeNotSortable(Language),

    #[error("Unexpected closing brace: {0}")]
    UnexpectedClosingBrace(char),

    #[error("Provided input not balanced")]
    InputNotBalanced,
}

pub type CsResult<T> = std::result::Result<T, CsError>;
