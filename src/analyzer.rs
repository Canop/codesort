use crate::*;

pub trait Analyzer {
    fn read<R: std::io::BufRead>(
        &self,
        reader: R,
    ) -> CsResult<LocList>;

    fn read_str(
        &self,
        s: &str,
    ) -> CsResult<LocList> {
        self.read(s.as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct CharSet {
    pub chars: Vec<char>,
}
impl From<char> for CharSet {
    fn from(c: char) -> Self {
        CharSet { chars: vec![c] }
    }
}
impl From<Vec<char>> for CharSet {
    fn from(chars: Vec<char>) -> Self {
        CharSet { chars }
    }
}

#[derive(Debug, Clone)]
pub struct Wish {
    pub any_of: CharSet,
    pub depth: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Gift {
    pub depth: usize,
    pub c: char,
}

impl Gift {
    pub fn satisfies(
        &self,
        wish: &Wish,
    ) -> bool {
        wish.depth == self.depth && wish.any_of.chars.contains(&self.c)
    }
}
