use crate::*;

/// An error in codesort
#[derive(thiserror::Error, Debug)]
pub enum CsError {
    #[error("Fmt error: {0}")]
    Fmt(#[from] std::fmt::Error), // only happens in debug

    #[error("Provided input not balanced")]
    InputNotBalanced,

    #[error("Invalid range {}..{}", .start+1, .end+1)]
    InvalidRange { start: LineIndex, end: LineIndex },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No sortable range found around line {}", .0+1)]
    NoSortableRangeAround(LineIndex),

    #[error("You can't specify both --around and --range")]
    RangeAndAround,

    #[error("Provided range not sortable (lang: {0:?})")]
    RangeNotSortable(Language),

    #[error("Unclosed char literal at line {}", .0+1)]
    UnclosedCharLiteral(LineIndex),

    #[error("Unexpected closing brace: {0}")]
    UnexpectedClosingBrace(char),
}

pub type CsResult<T> = std::result::Result<T, CsError>;
