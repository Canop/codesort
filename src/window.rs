use crate::*;

/// Line indices are 0-based.
pub struct Window {
    /// The lines of the complete file
    pub list: List,
    /// index in the list of the first line of the winow
    pub start: usize,
    /// index of the first not included line, end >= start
    pub end: usize,
}

impl Window {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
    pub fn blocks(&self) -> CsResult<Vec<Block>> {
        let mut blocks = Vec::new();
        let mut current_block = None;
        for line_idx in self.start..self.end {
            let Some(block) = current_block.as_mut() else {
                let block = Block::new(line_idx, &self.list);
                if block.is_complete() {
                    blocks.push(block);
                } else {
                    current_block = Some(block);
                }
                continue;
            };
            block.augment(&self.list);
            if block.is_complete() && !block.is_empty() {
                blocks.push(current_block.take().unwrap());
            }
        }
        if let Some(block) = current_block {
            if !block.is_balanced() {
                return Err(CsError::RangeNotSortable);
            }
            blocks.push(block);
        }
        // before returning, we merge annotation blocks
        let mut merged_blocks: Vec<Block> = Vec::new();
        for mut block in blocks.drain(..) {
            if merged_blocks
                .last()
                .map_or(false, |last| last.is_annotation())
            {
                let last = merged_blocks.pop().unwrap();
                block.start = last.start;
                block.code = last.code + &block.code;
            }
            merged_blocks.push(block);
        }
        Ok(merged_blocks)
    }
    pub fn sort_blocks(
        &self,
        blocks: &mut [Block],
    ) {
        blocks.sort_by(|a, b| a.sort_key().cmp(b.sort_key()));
    }
    pub fn sort(self) -> CsResult<List> {
        let lang = self.list.lang;
        let mut blocks = self.blocks()?;
        self.sort_blocks(&mut blocks);
        let mut src_lines: Vec<_> = self.list.lines.into_iter().map(Some).collect();
        let mut lines = Vec::with_capacity(src_lines.len());
        for line in &mut src_lines[..self.start] {
            lines.push(line.take().unwrap());
        }
        for block in &blocks {
            for line in &mut src_lines[block.start()..block.end()] {
                lines.push(line.take().unwrap());
            }
        }
        for line in &mut src_lines[self.end..] {
            lines.push(line.take().unwrap());
        }
        Ok(List { lines, lang })
    }
}
