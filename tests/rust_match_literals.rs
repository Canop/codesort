use codesort::*;

#[test]
fn test_match_literals() {
    static INPUT: &str = r#"
        pub fn analyze(aliment: &str) -> Kind {
            match aliment {
                "pepper" => Kind::Fruit, // I guess
                "apple" => Kind::Fruit,
                // well...
                "tomato" => if rand::random() {
                    Kind::Vegetable
                } else {
                    Kind::Fruit
                },
                "carrot" => Kind::Vegetable,
                "avocado" => Kind::Both,
                // There should probably be another kind for this one
                "samphire" => Kind::Unknown,
                _ => Kind::Unknown,
            }
        }
    "#;

    static OUTPUT: &str = r#"
        pub fn analyze(aliment: &str) -> Kind {
            match aliment {
                "apple" => Kind::Fruit,
                "avocado" => Kind::Both,
                "carrot" => Kind::Vegetable,
                "pepper" => Kind::Fruit, // I guess
                // There should probably be another kind for this one
                "samphire" => Kind::Unknown,
                // well...
                "tomato" => if rand::random() {
                    Kind::Vegetable
                } else {
                    Kind::Fruit
                },
                _ => Kind::Unknown,
            }
        }
    "#;

    let analyzer = RustAnalyzer;
    let list = analyzer.read_str(INPUT).unwrap();
    let focused = list.focus_around_line_idx(6).unwrap();
    focused.print_debug();
    {
        let blocks = focused.clone().focus.into_blocks();
        for (i, block) in blocks.iter().enumerate() {
            block.print_debug(&format!(" BLOCK {i}"));
        }
        assert!(blocks[1] < blocks[0]);
        assert!(blocks[3] < blocks[0]);
    }
    let sorted_list = focused.sort();
    sorted_list.print_debug(" SORTED ");
    assert_eq!(sorted_list.to_string(), OUTPUT);
}
