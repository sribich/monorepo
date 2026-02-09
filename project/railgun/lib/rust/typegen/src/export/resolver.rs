use std::collections::HashMap;
use std::collections::HashSet;

use crate::id::TypeId;

#[derive(Debug, Default)]
pub struct ExportResolver {
    types: HashMap<TypeId, Export>,
    names: HashSet<String>,
}

impl ExportResolver {
    pub fn new() -> Self {
        Self {
            types: HashMap::default(),
            names: HashSet::default(),
        }
    }

    pub fn resolve(&mut self, name: &str, id: TypeId) -> Export {
        let export = Export {
            id,
            name: name.to_owned(),
        };

        self.types.insert(id, export);
        self.names.insert(name.to_owned());

        self.types.get(&id).unwrap().clone()
    }

    pub fn get(&self, item: &TypeId) -> Option<&Export> {
        self.types.get(item)
    }

    pub fn contains(&self, item: &TypeId) -> bool {
        self.types.contains_key(item)
    }
}

#[derive(Debug, Clone)]
pub struct Export {
    pub id: TypeId,
    pub name: String,
}

impl Export {}
