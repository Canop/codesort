use crate::*;

/// An error in code-sort
#[derive(thiserror::Error, Debug)]
pub enum CsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("No sortable range found around line number {}", .0+1)]
    NoSortableRangeAround(LineIndex),

    #[error("Invalid range {start}..{end} (0-based)")]
    InvalidRange { start: usize, end: usize },

    #[error("You can't specify both --around and --range")]
    RangeAndAround,
}

pub type CsResult<T> = std::result::Result<T, CsError>;
