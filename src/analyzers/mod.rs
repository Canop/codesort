use {
    crate::*,
    std::path::Path,
};

pub mod java;
pub mod javascript;
pub mod rust;

/// The language syntax to use for analyzing the code
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    C,
    Java,
    Javascript,
    #[default]
    Rust,
    Zig,
}

impl Language {
    pub fn detect(path: &Path) -> Option<Language> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "rs" => Some(Language::Rust),
            "java" => Some(Language::Java),
            "js" => Some(Language::Javascript),
            _ => None,
        }
    }
    pub fn analyzer(self) -> Analyzer {
        match self {
            Self::C => Analyzer::Rust, // should be OK
            Self::Java => Analyzer::Java,
            Self::Javascript => Analyzer::Javascript,
            Self::Rust => Analyzer::Rust,
            Self::Zig => Analyzer::Rust, // should be OK
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Analyzer {
    Java,
    Javascript,
    /// Rust, C and Zig are analyzed the same way
    Rust,
}

impl Analyzer {
    pub fn read<R: std::io::BufRead>(
        &self,
        mut reader: R,
    ) -> CsResult<LocList> {
        match self {
            Self::Java => java::read(&mut reader),
            Self::Javascript => javascript::read(&mut reader),
            Self::Rust => rust::read(&mut reader),
        }
    }
}
