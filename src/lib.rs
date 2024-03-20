//!
//! codesort sorts code, either as a command line tool, usually integrated
//! in an IDE or a text editor, or as a library.
//!
//! ```rust
//! use codesort::{List, Language};
//! let input = r#"
//! pub enum ContentSearchResult {
//!
//!     /// the file wasn't searched because it's binary or too big
//!     NotSuitable,
//!
//!     /// the needle has been found at the given pos
//!     Found {
//!         pos: usize,
//!     },
//!
//!     /// the needle hasn't been found
//!     NotFound, // no match
//! }
//! "#;
//!
//! let output = r#"
//! pub enum ContentSearchResult {
//!
//!     /// the needle has been found at the given pos
//!     Found {
//!         pos: usize,
//!     },
//!
//!     /// the needle hasn't been found
//!     NotFound, // no match
//!
//!     /// the file wasn't searched because it's binary or too big
//!     NotSuitable,
//! }
//! "#;
//!
//! let list = List::from_str(input, Language::Rust).unwrap();
//! let window = list.window_around(6).unwrap();
//! assert_eq!(
//!     window.sort().unwrap().to_string(),
//!     output,
//! );
//! ```

mod analyzers;
mod balanced;
mod block;
mod error;
mod languages;
mod line;
mod line_number;
mod list;
mod tprint;
mod window;

pub use {
    analyzers::*,
    balanced::*,
    block::*,
    error::*,
    languages::*,
    line::*,
    line_number::*,
    list::*,
    window::*,
};
