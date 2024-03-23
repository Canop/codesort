// TODO try to explain how and why this works without sounding too crazy

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

/// Something that is required by the state of a LOC list
#[derive(Debug, Clone)]
pub struct Wish {
    pub any_of: CharSet,
    pub depth: usize,
}

/// Something that can maybe satisfy a wish
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
