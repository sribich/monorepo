use std::collections::HashSet;

use dictionary_parser::Accent;
use dictionary_parser::Frequency;
use features::shared::domain::value::muid::Muid;

use crate::feature::dictionary::domain::cdc::dictionary::DictionaryChangeEvent;
use crate::feature::dictionary::domain::cdc::dictionary::DictionaryChangeUniqueEvent;
use crate::feature::dictionary::domain::entity::definition::Definition;
use crate::feature::dictionary::domain::value::dictionary_kind::DictionaryKind;
use crate::feature::dictionary::domain::value::language_type::LanguageType;

#[derive(Clone)]
pub struct Dictionary {
    id: Muid,
    title: String,

    language_type: LanguageType,
    kinds: HashSet<DictionaryKind>,

    file_path: String,
    data_path: String,

    definitions: Vec<Definition>,
    accents: Vec<Accent>,
    frequencies: Vec<Frequency>,

    _changes: Vec<DictionaryChangeEvent>,
    _uniqueChanges: HashSet<DictionaryChangeUniqueEvent>,
}

impl Dictionary {
    pub fn new(
        id: Option<Muid>,
        title: String,
        language_type: LanguageType,
        file_path: String,
        data_path: String,
    ) -> Self {
        let mut this = Self {
            id: id.unwrap_or_else(Muid::new_now),
            title,
            language_type,
            kinds: Default::default(),
            file_path,
            data_path,
            definitions: vec![],
            accents: vec![],
            frequencies: vec![],

            _changes: vec![],
            _uniqueChanges: Default::default(),
        };

        this._changes
            .push(DictionaryChangeEvent::Created(this.clone()));

        this
    }

    pub fn id(&self) -> &Muid {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn kinds(&self) -> Vec<DictionaryKind> {
        self.kinds.iter().map(Clone::clone).collect::<Vec<_>>()
    }

    pub fn language_type(&self) -> &LanguageType {
        &self.language_type
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn data_path(&self) -> &str {
        &self.data_path
    }

    pub fn add_definitions(&mut self, mut words: Vec<Definition>) {
        if words.is_empty() {
            println!("empty");
            return;
        }

        self._changes
            .push(DictionaryChangeEvent::AddDefinitions(words.clone()));
        self.definitions.append(&mut words);

        self.kinds.insert(DictionaryKind::Word);
        self._uniqueChanges
            .insert(DictionaryChangeUniqueEvent::ChangedKinds);
    }

    pub fn add_frequencies(&mut self, mut frequencies: Vec<Frequency>) {
        if frequencies.is_empty() {
            return;
        }

        self._changes
            .push(DictionaryChangeEvent::AddFrequencies(frequencies.clone()));
        self.frequencies.append(&mut frequencies);

        self.kinds.insert(DictionaryKind::Frequency);
        self._uniqueChanges
            .insert(DictionaryChangeUniqueEvent::ChangedKinds);
    }

    pub fn add_accents(&mut self, mut accents: Vec<Accent>) {
        if accents.is_empty() {
            return;
        }

        self._changes
            .push(DictionaryChangeEvent::AddAccents(accents.clone()));
        self.accents.append(&mut accents);

        self.kinds.insert(DictionaryKind::PitchAccent);
        self._uniqueChanges
            .insert(DictionaryChangeUniqueEvent::ChangedKinds);
    }

    pub fn changes(&mut self) -> Vec<DictionaryChangeEvent> {
        std::mem::take(&mut self._changes)
    }

    pub fn unique_changes(&mut self) -> Vec<DictionaryChangeUniqueEvent> {
        std::mem::take(&mut self._uniqueChanges)
            .into_iter()
            .collect::<Vec<_>>()
    }

    //     pub fn info(&self) -> &DictionaryInfo {
    //         &self.info
    //     }
    //
    //     pub fn info_owned(self) -> DictionaryInfo {
    //         self.info
    //     }
}
