use {
    code_sort::*,
    std::fmt::Write,
};

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

    let list = List::from_str(INPUT, Language::Rust).unwrap();
    let window = list.window_around(6).unwrap();

    let blocks = window.blocks().unwrap();
    tprintln!("blocks:");
    for block in blocks {
        tprint!("\n------\n{}", block.sort_key());
        //tprint!("\n------\n{}", block.content());
    }

    let mut output = String::new();
    write!(output, "{}", window.sort().unwrap()).unwrap();
    println!("{}", output);
    assert_eq!(output, OUTPUT);
}
