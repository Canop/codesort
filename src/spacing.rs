use crate::*;

/// The kind of spacing between blocs
///
/// We're only interested, for now, in the specific "Between" kind, which
/// is the only one which is broken by raw sorting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spacing {
    /// at least one blank line between every two blocs, and none
    /// before the first one
    Between,
    Other,
}

impl Spacing {
    pub fn recognize(blocks: &[LocList]) -> Spacing {
        let mut blocks = blocks.iter();
        let Some(first) = blocks.next() else {
            return Self::Other;
        };
        if first.count_blank_lines_at_start() > 0 {
            return Self::Other;
        }
        if blocks.all(|bloc| bloc.count_blank_lines_at_start() > 0) {
            Self::Between
        } else {
            Self::Other
        }
    }
    pub fn apply(
        self,
        blocks: &mut [LocList],
    ) {
        match self {
            Self::Between => {
                // To restore in between spacing, we take the unwanted blank
                // lines heading the first block and we move them to a block
                // having none
                let Some(first_block) = blocks.first_mut() else {
                    return;
                };
                let count_on_first = first_block.count_blank_lines_at_start();
                if count_on_first == 0 {
                    return;
                }
                let blank_lines: Vec<Loc> =
                    first_block.locs.drain(0..count_on_first).collect();
                let Some(block_without) = blocks
                    .iter_mut()
                    .skip(1)
                    .find(|bloc| bloc.count_blank_lines_at_start() == 0)
                else {
                    return;
                };
                for loc in blank_lines {
                    block_without.locs.insert(0, loc);
                }
            }
            Self::Other => {}
        }
    }
}

#[test]
fn test_restore_spacing_between() {
    let input = r#"
        pub enum CsError {
            #[error("You can't specify both --around and --range")]
            RangeAndAround,

            #[error("IO error: {0}")]
            Io(#[from] std::io::Error),

            #[error("Fmt error: {0}")]
            Fmt(#[from] std::fmt::Error), // only happens in debug

            #[error("No sortable range found around line {}", .0+1)]
            NoSortableRangeAround(LineIndex),

            #[error("Invalid range {}..{}", .start+1, .end+1)]
            InvalidRange { start: LineIndex, end: LineIndex },

            #[error("Provided range not sortable (lang: {0:?})")]
            RangeNotSortable(Language),

            #[error("Unexpected closing brace: {0}")]
            UnexpectedClosingBrace(char),

            #[error("Unclosed char literal")]
            UnclosedCharLiteral,

            #[error("Provided input not balanced")]
            InputNotBalanced,
        }
    "#;
    let list = LocList::read_str(input, Language::Rust).unwrap();
    let focused = list.focus_around_line_index(4).unwrap();
    focused.print_debug();

    // blocks are initially separated by a blank line
    let mut blocks = focused.clone().focus.into_blocks();
    let spacing = Spacing::recognize(&blocks);
    assert_eq!(spacing, Spacing::Between);

    // check that simple sort of the blocks breask the spacing
    blocks.sort();
    assert_eq!(Spacing::recognize(&blocks), Spacing::Other);

    // restore the spacing, and check
    spacing.apply(&mut blocks);
    assert_eq!(Spacing::recognize(&blocks), Spacing::Between);

    // now check that calling sort on the focused automatically restores
    // the spacing
    let output = r#"
        pub enum CsError {
            #[error("Fmt error: {0}")]
            Fmt(#[from] std::fmt::Error), // only happens in debug

            #[error("Provided input not balanced")]
            InputNotBalanced,

            #[error("Invalid range {}..{}", .start+1, .end+1)]
            InvalidRange { start: LineIndex, end: LineIndex },

            #[error("IO error: {0}")]
            Io(#[from] std::io::Error),

            #[error("No sortable range found around line {}", .0+1)]
            NoSortableRangeAround(LineIndex),

            #[error("You can't specify both --around and --range")]
            RangeAndAround,

            #[error("Provided range not sortable (lang: {0:?})")]
            RangeNotSortable(Language),

            #[error("Unclosed char literal")]
            UnclosedCharLiteral,

            #[error("Unexpected closing brace: {0}")]
            UnexpectedClosingBrace(char),
        }
    "#;
    let sorted_list = focused.sort();
    assert_eq!(sorted_list.to_string(), output);
}
