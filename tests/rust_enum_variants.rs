use codesort::*;

#[test]
fn test_enum_variants_simple_with_spaces() {
    static INPUT: &str = r#"
    /// result of a full text search
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ContentSearchResult {

        /// the file wasn't searched because it's binary or too big
        NotSuitable,

        /// the needle has been found at the given pos
        Found {
            pos: usize,
        },

        /// the needle hasn't been found
        NotFound, // no match
    }
    "#;

    static OUTPUT: &str = r#"
    /// result of a full text search
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ContentSearchResult {

        /// the needle has been found at the given pos
        Found {
            pos: usize,
        },

        /// the needle hasn't been found
        NotFound, // no match

        /// the file wasn't searched because it's binary or too big
        NotSuitable,
    }
    "#;

    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    //list.print_debug(" WHOLE ");
    let focused = list.focus_around_line_index(6).unwrap();
    focused.print_debug();
    {
        let blocks = focused.clone().focus.into_blocks();
        for (i, block) in blocks.iter().enumerate() {
            block.print_debug(&format!(" BLOCK {i}"));
        }
    }
    let sorted_list = focused.sort();
    sorted_list.print_debug(" SORTED ");
    assert_eq!(sorted_list.to_string(), OUTPUT);
}

#[test]
fn test_enum_variants_simple_without_space() {
    static INPUT: &str = r#"
    #[derive(Clone, Debug, Copy, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum SpecialHandlingShortcut {
        None,
        Enter,
        #[serde(alias = "no-enter")]
        NoEnter,
        Hide,
        #[serde(alias = "no-hide")]
        NoHide,
    }
    "#;

    static OUTPUT: &str = r#"
    #[derive(Clone, Debug, Copy, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum SpecialHandlingShortcut {
        Enter,
        Hide,
        #[serde(alias = "no-enter")]
        NoEnter,
        #[serde(alias = "no-hide")]
        NoHide,
        None,
    }
    "#;

    let mut list = LocList::read_str(INPUT, Language::Rust).unwrap();
    list.sort_around_line_index(6).unwrap();
    assert_eq!(list.to_string(), OUTPUT);
}
