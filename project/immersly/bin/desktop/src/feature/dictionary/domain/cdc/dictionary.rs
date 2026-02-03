use dictionary_parser::Accent;
use dictionary_parser::Frequency;

use crate::feature::dictionary::domain::aggregate::dictionary::Dictionary;
use crate::feature::dictionary::domain::entity::definition::Definition;

#[derive(Clone)]
pub enum DictionaryChangeEvent {
    Created(Dictionary),
    AddDefinitions(Vec<Definition>),
    AddAccents(Vec<Accent>),
    AddFrequencies(Vec<Frequency>),
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DictionaryChangeUniqueEvent {
    ChangedKinds,
}
