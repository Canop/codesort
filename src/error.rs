/// An error in code-sort
#[derive(thiserror::Error, Debug)]
pub enum CsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),
}

pub type CsResult<T> = std::result::Result<T, CsError>;
