use codesort::*;

static INPUT: &str = r#"
trait Foo {}

struct Bar<T>(T);
struct Baz<T>(T);
struct Blee<T>(T);

impl<T> Foo for Blee<T>
where
    T: Copy,
{
    // ...
}

impl<T> Foo for Baz<T>
where
    T: Copy,
{
    // ...
}

impl<T> Foo for Bar<T>
where
    T: Copy,
{
    // ...
}

fn main() {}
"#;

static OUTPUT: &str = r#"
fn main() {}

impl<T> Foo for Bar<T>
where
    T: Copy,
{
    // ...
}

impl<T> Foo for Baz<T>
where
    T: Copy,
{
    // ...
}

impl<T> Foo for Blee<T>
where
    T: Copy,
{
    // ...
}

struct Bar<T>(T);
struct Baz<T>(T);
struct Blee<T>(T);

trait Foo {}
"#;

/// check issue #4 is solved
#[test]
fn test_comma_in_trait_bound() {
    let list = LocList::read_str(INPUT, Language::Rust).unwrap();
    //list.print_debug(" WHOLE ");
    let focused = list.focus_around_line_index(7).unwrap();
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
