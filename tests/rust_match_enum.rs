use codesort::*;

#[test]
fn test_match_enum() {
    static INPUT: &str = r#"
    impl From<SpecialHandlingShortcut> for SpecialHandling {
        fn from(shortcut: SpecialHandlingShortcut) -> Self {
            use Directive::*;
            match shortcut {
                SpecialHandlingShortcut::None => SpecialHandling {
                    show: Default, list: Default, sum: Default,
                },
                SpecialHandlingShortcut::Enter => SpecialHandling {
                    show: Always, list: Always, sum: Always,
                },
                SpecialHandlingShortcut::NoEnter => SpecialHandling {
                    show: Default, list: Never, sum: Never,
                },
                SpecialHandlingShortcut::Hide => SpecialHandling {
                    show: Never, list: Default, sum: Never,
                },
                SpecialHandlingShortcut::NoHide => SpecialHandling {
                    show: Always, list: Default, sum: Default,
                },
            }
        }
    }
    "#;
    static OUTPUT: &str = r#"
    impl From<SpecialHandlingShortcut> for SpecialHandling {
        fn from(shortcut: SpecialHandlingShortcut) -> Self {
            use Directive::*;
            match shortcut {
                SpecialHandlingShortcut::Enter => SpecialHandling {
                    show: Always, list: Always, sum: Always,
                },
                SpecialHandlingShortcut::Hide => SpecialHandling {
                    show: Never, list: Default, sum: Never,
                },
                SpecialHandlingShortcut::NoEnter => SpecialHandling {
                    show: Default, list: Never, sum: Never,
                },
                SpecialHandlingShortcut::NoHide => SpecialHandling {
                    show: Always, list: Default, sum: Default,
                },
                SpecialHandlingShortcut::None => SpecialHandling {
                    show: Default, list: Default, sum: Default,
                },
            }
        }
    }
    "#;
    let analyzer = RustAnalyzer;
    let list = analyzer.read_str(INPUT).unwrap();
    let focused = list.focus_around_line_idx(7).unwrap();
    let sorted_list = focused.sort();
    assert_eq!(sorted_list.to_string(), OUTPUT);
}
