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

    let list = List::from_str(INPUT, Language::Rust).unwrap();
    let window = list.window_around(6).unwrap();
    dbg!((window.start, window.end));
    assert_eq!(window.len(), 11);
    assert_eq!(window.sort().unwrap().to_string(), OUTPUT,);
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

    let list = List::from_str(INPUT, Language::Rust).unwrap();
    let window = list.window_around(6).unwrap();
    assert_eq!(window.sort().unwrap().to_string(), OUTPUT,);
}
