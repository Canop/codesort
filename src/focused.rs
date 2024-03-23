use crate::*;

#[derive(Debug, Clone)]
pub struct Focused {
    pub before: LocList,
    pub focus: LocList,
    pub after: LocList,
}

impl Focused {
    pub fn print_debug(&self) {
        self.before.print_debug(" BEFORE ");
        self.focus.print_debug(" FOCUS ");
        self.after.print_debug(" AFTER ");
    }
    pub fn sort(self) -> LocList {
        let mut locs = self.before.locs;
        let mut blocks = self.focus.into_blocks();
        blocks.sort();
        for block in blocks {
            locs.extend(block.locs);
        }
        locs.extend(self.after.locs);
        LocList { locs }
    }
}
