pub mod javascript;
pub mod rust;

use {
    crate::*,
    std::str::FromStr,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    /// Should also work for C, and maybe others
    #[default]
    Rust,
    JavaScript,
}
pub static LANGUAGES: &[Language] = &[Language::Rust, Language::JavaScript];

impl Language {
    pub fn check_balanced(
        &self,
        s: &str,
    ) -> Option<Balanced> {
        match self {
            Language::Rust => rust::check_balanced(s),
            Language::JavaScript => javascript::check_balanced(s),
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

impl FromStr for Language {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            "javascript" | "js" => Ok(Language::JavaScript),
            _ => Err(format!("unknown language: {}", s)),
        }
    }
}
