pub mod java;
pub mod javascript;
pub mod rust;

use {
    crate::*,
    std::path::Path,
};


/// The language syntax to use for analyzing the code
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    /// Should also work for C, Zig, and probably some other ones
    #[default]
    Rust,
    /// It should work, but I didn't do much Java in recent years
    Java,
    /// Should also work for TypeScript
    JavaScript,
}

impl Language {
    pub fn check_balanced(
        &self,
        s: &str,
    ) -> Option<Balanced> {
        match self {
            Language::Rust => rust::check_balanced(s),
            Language::Java => java::check_balanced(s),
            Language::JavaScript => javascript::check_balanced(s),
        }
    }
    pub fn detect(path: &Path) -> Option<Language> {
        let ext = path.extension()?.to_str()?;
        match ext {
            "rs" => Some(Language::Rust),
            "java" => Some(Language::Java),
            "js" => Some(Language::JavaScript),
            _ => None,
        }
    }
}

fn char_is_brace(c: u8) -> bool {
    match c {
        b'{' | b'}' | b'[' | b']' | b'(' | b')' => true,
        _ => false,
    }
}
fn braces_are_balanced(braces: &[u8]) -> bool {
    let len = braces.len();
    if len % 2 != 0 {
        return false;
    }
    let mut stack = Vec::new();
    for &brace in braces {
        match brace {
            b'(' | b'[' | b'{' => stack.push(brace),
            b')' => match stack.pop() {
                Some(b'(') => (),
                _ => return false,
            },
            b']' => match stack.pop() {
                Some(b'[') => (),
                _ => return false,
            },
            b'}' => match stack.pop() {
                Some(b'{') => (),
                _ => return false,
            },
            _ => panic!("unexpected brace: {}", brace as char),
        }
    }
    stack.is_empty()
}
