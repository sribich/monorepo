use typegen::{NamedType, Typegen};

#[cfg(test)]
#[test]
fn basic_output() {
    #[derive(Typegen)]
    struct Test {
        name: String,
    }
}
