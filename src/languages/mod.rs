mod rust;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {
    /// Should also work for C and Java, and maybe others
    Rust,
    JavaScript,
}
pub static LANGUAGES: &[Language] = &[Language::Rust, Language::JavaScript];

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

/// A piece of code made of complete lines, with balanced braces
/// and some significant content
#[derive(Debug, Clone)]
pub struct Balanced {
    //pub braces: Vec<u8>,
    pub last_significant_char: char,
    pub language: Language,
}

impl Balanced {
    pub fn new(code: &str) -> Option<Self> {
        rust::check_balanced_rust(code)
    }
}
