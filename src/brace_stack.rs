use crate::*;

#[derive(Debug, Clone, Default)]
pub struct BraceStack {
    pub braces: Vec<char>,
}

impl BraceStack {
    pub fn push(
        &mut self,
        brace: char,
    ) -> CsResult<()> {
        match brace {
            '(' | '[' | '{' => self.braces.push(brace),
            ')' => {
                if self.braces.pop() != Some('(') {
                    return Err(CsError::UnexpectedClosingBrace(brace));
                }
            }
            ']' => {
                if self.braces.pop() != Some('[') {
                    return Err(CsError::UnexpectedClosingBrace(brace));
                }
            }
            '}' => {
                if self.braces.pop() != Some('{') {
                    return Err(CsError::UnexpectedClosingBrace(brace));
                }
            }
            _ => panic!("unexpected brace: {}", brace),
        }
        Ok(())
    }
    pub fn depth(&self) -> usize {
        self.braces.len()
    }
}

pub fn char_is_brace(c: char) -> bool {
    match c {
        '{' | '}' | '[' | ']' | '(' | ')' => true,
        _ => false,
    }
}
