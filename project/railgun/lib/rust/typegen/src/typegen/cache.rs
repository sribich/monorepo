// TODO: Use a typestate here for a resolved & unresolved, do not let inserting
// in resolved state but only setting exported stats.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::datatype::NamedDataType;
use crate::id::TypeId;

#[allow(clippy::large_enum_variant, reason = "Not a performance bottleneck")]
#[derive(Clone, Debug)]
pub enum CachedType {
    InProgress,
    Resolved(NamedDataType),
}

impl Default for CachedType {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(Clone, Debug)]
pub struct ExportIdentifier {
    pub id: TypeId,

    /// The name of the module that the export lives in.
    pub in_module: String,
    /// Modules that need this export.
    pub dependent_modules: HashSet<String>,

    pub content: Option<String>,
}

impl ExportIdentifier {
    pub fn new(id: TypeId, module: &str) -> Self {
        Self {
            id,
            in_module: module.to_owned(),
            dependent_modules: HashSet::from_iter(vec![module.to_owned()]),
            content: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TypeCache {
    pub cache: BTreeMap<TypeId, CachedType>,

    /// Handles module based generation where we need to keep track of
    /// where an individual file may have been exported.
    pub export_cache: RefCell<BTreeMap<TypeId, ExportIdentifier>>,
}

impl TypeCache {
    // pub fn maybe_export(&self, id: &TypeId) -> Option<&NamedDataType> {
    //     if self.is_exported(id) {
    //         return None;
    //     }
    //
    //     self.set_exported(id);
    //     self.get(id)
    // }

    pub fn is_exported(&self, id: &TypeId) -> bool {
        self.export_cache.borrow().contains_key(id)
    }

    pub fn set_exported(&self, id: &TypeId, module: ExportIdentifier) {
        self.export_cache.borrow_mut().insert(*id, module);
    }

    pub fn add_export_dependency(&self, id: &TypeId, on: String) {
        if let Some(value) = self.export_cache.borrow_mut().get_mut(id) {
            value.dependent_modules.insert(on);
        }
    }

    pub fn set_export_content(&self, id: &TypeId, content: String) {
        if let Some(value) = self.export_cache.borrow_mut().get_mut(id) {
            value.content = Some(content);
        }
    }

    pub fn get_exports(&self) -> BTreeMap<TypeId, ExportIdentifier> {
        std::mem::take(&mut self.export_cache.borrow_mut())
    }

    pub fn contains(&self, id: &TypeId) -> bool {
        self.cache.contains_key(id)
    }

    pub fn get(&self, id: &TypeId) -> Option<&NamedDataType> {
        let result = self.cache.get(id)?;

        match result {
            CachedType::InProgress => None,
            CachedType::Resolved(ty) => Some(ty),
        }
    }

    pub fn insert(&mut self, id: TypeId, cached_type: CachedType) {
        self.cache.insert(id, cached_type);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &CachedType)> {
        self.cache.iter()
    }
}

impl IntoIterator for TypeCache {
    type IntoIter = <BTreeMap<TypeId, CachedType> as IntoIterator>::IntoIter;
    type Item = (TypeId, CachedType);

    fn into_iter(self) -> Self::IntoIter {
        self.cache.into_iter()
    }
}
