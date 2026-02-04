/// TODO: Document
///
/// Allowing camelCase makes it easier to generate this type from
/// macros since we do not need to do any kind of mapping. We will
/// be able to construct the enum from the `ty` directly.
#[expect(
    non_camel_case_types,
    reason = "Variants reflect their actual rust type"
)]
#[derive(Clone, Debug)]
pub enum PrimitiveMeta {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    bool,
    char,
    String,
}
