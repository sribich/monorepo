use std::collections::HashSet;

use dictionary_parser::Accent;
use dictionary_parser::Frequency;
use shared::entity_data_fns;

use crate::domain::value::dictionary_id::DictionaryId;
use crate::domain::value::dictionary_kind::DictionaryKind;
use crate::domain::value::language_type::LanguageType;

#[derive(Clone)]
pub struct DictionaryData {
    pub id: DictionaryId,
    pub title: String,

    pub language_type: LanguageType,
    pub kinds: HashSet<DictionaryKind>,

    pub file_path: String,
    pub data_path: String,
}

#[derive(Clone)]
pub struct Dictionary(DictionaryData);

entity_data_fns!(Dictionary);

impl Dictionary {
    pub fn id(&self) -> &DictionaryId {
        &self.0.id
    }

    pub fn title(&self) -> &str {
        &self.0.title
    }

    pub fn kinds(&self) -> Vec<DictionaryKind> {
        self.0.kinds.iter().map(Clone::clone).collect::<Vec<_>>()
    }

    pub fn language_type(&self) -> &LanguageType {
        &self.0.language_type
    }

    pub fn file_path(&self) -> &str {
        &self.0.file_path
    }

    pub fn data_path(&self) -> &str {
        &self.0.data_path
    }
}
