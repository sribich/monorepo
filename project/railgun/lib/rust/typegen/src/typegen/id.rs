#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId {
    // pub path: &'static str,
    // pub line: u32,
    // pub col: u32,
    pub name: &'static str,
    pub hash: u64,
}

impl TypeId {
    pub const fn delegate(self, from: TypeId) -> TypeId {
        let mut new = self;

        new.name = from.name;

        new
    }
}

/// SOURCE_LOCATION points to the position at which the derivation
/// symbol is defined in the original source.
///
/// #[derive(Type)]
///          ^--- Location points here
#[derive(Clone, Copy, Debug)]
pub struct SourceLocation(pub &'static str);
