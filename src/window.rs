use crate::*;

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
    pub fn blocks(&self) -> Vec<Block> {
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
            if block.is_complete() {
                blocks.push(current_block.take().unwrap());
            }
        }
        if let Some(block) = current_block {
            blocks.push(block);
        }
        blocks
    }
    pub fn sort_blocks(
        &self,
        blocks: &mut [Block],
    ) {
        blocks.sort_by(|a, b| {
            let mut ai = a.start();
            let mut bi = b.start();
            while ai < self.end && self.list.lines[ai].exclude_from_sort() {
                ai += 1;
            }
            while bi < self.end && self.list.lines[bi].exclude_from_sort() {
                bi += 1;
            }
            while ai < a.end() && bi < b.end() {
                match self.list.lines[ai].inner().cmp(self.list.lines[bi].inner()) {
                    std::cmp::Ordering::Equal => {
                        ai += 1;
                        bi += 1;
                    }
                    other => return other,
                }
            }
            std::cmp::Ordering::Equal
        });
    }
    pub fn sort(self) -> List {
        let mut blocks = self.blocks();
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
        List { lines }
    }
}
