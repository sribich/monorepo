use typegen::NamedType;
use typegen::Typegen;

#[cfg(test)]
#[test]
fn basic_output() {
    #[derive(Typegen)]
    struct Test {
        name: String,
    }
}
