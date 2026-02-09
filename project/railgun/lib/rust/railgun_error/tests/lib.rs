use railgun_error::Error;
use railgun_error::ResultExt;

#[derive(Error)]
enum Error {
    VariantA { name: String, age: u32 },
}

#[test]
fn test() {
    let x: Result<(), Error> = VariantAContext {
        name: "wee",
        age: 10_u32,
    }
    .fail();
}
