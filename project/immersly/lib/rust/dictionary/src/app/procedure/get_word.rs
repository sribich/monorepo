use std::collections::HashMap;
use std::sync::Arc;

use prisma_client::model::SortOrder;
use prisma_client::model::{self};
use railgun::di::Component;
use railgun::typegen::Typegen;
use serde::Deserialize;
use serde::Serialize;
use shared::domain::value::muid::Muid;
use shared::infra::Procedure;
use shared::infra::database::Sqlite;

#[derive(Debug, Deserialize, Typegen)]
pub struct GetWordQueryRequest {
    pub word: String,
}

#[derive(Debug, Serialize, Typegen)]
pub struct GetWordQueryResponse {
    definitions: Vec<DefinitionEntry>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct DefinitionEntry {
    word: String,
    reading: String,

    definitions: Vec<Definition>,

    frequencies: Vec<Freq>,
    accents: Vec<Accent>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct Definition {
    id: String,
    dictionary: String,
    definition: String,
}

#[derive(Debug, Serialize, Typegen)]
pub struct Freq {
    id: String,
    dictionary: String,
    freq: String,
}

#[derive(Debug, Serialize, Typegen)]
pub struct Accent {
    id: String,
    dictionary: String,
    reading: String,
    accent: u8,
}

#[derive(Component)]
pub struct GetWordProcedure {
    db: Arc<Sqlite>,
}

impl Procedure for GetWordProcedure {
    type Err = core::convert::Infallible;
    type Req = GetWordQueryRequest;
    type Res = GetWordQueryResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let client = self.db.client();
        let db = Arc::clone(&self.db);

        let mut words = client
            .word()
            .find_many(vec![model::word::word::equals(data.word.clone())])
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await
            .unwrap();

        if words.is_empty() {
            let result = client
                .word()
                .find_many(vec![model::word::reading::equals(data.word.clone())])
                .with(model::word::dictionary::fetch())
                .order_by(model::word::dictionary::order(vec![
                    model::dictionary::rank::order(SortOrder::Asc),
                ]))
                .exec()
                .await
                .unwrap();

            words.extend_from_slice(&result);
        }

        let mut hashmap: HashMap<(String, String), DefinitionEntry> = HashMap::new();

        for word in words {
            if let Some(entry) = hashmap.get_mut(&(word.word.clone(), word.reading.clone())) {
                entry.definitions.push(Definition {
                    id: Muid::try_from_slice(&word.id).unwrap().to_string(),
                    dictionary: word.dictionary.unwrap().title,
                    definition: word.definition,
                });
            } else {
                hashmap.insert(
                    (word.word.clone(), word.reading.clone()),
                    DefinitionEntry {
                        word: word.word,
                        reading: word.reading,
                        definitions: vec![Definition {
                            id: Muid::try_from_slice(&word.id).unwrap().to_string(),
                            dictionary: word.dictionary.unwrap().title,
                            definition: word.definition,
                        }],
                        frequencies: vec![],
                        accents: vec![],
                    },
                );
            }
        }

        let words = hashmap
            .into_values()
            .map(|entry| {
                let db = Arc::clone(&db);

                tokio::spawn(async move {
                    let word = entry.word.clone();
                    let reading = entry.reading.clone();

                    let frequencies = db
                        .client()
                        .frequency()
                        .find_many(vec![
                            model::frequency::word::equals(word.clone()),
                            model::frequency::reading::equals(reading.clone()),
                        ])
                        .with(model::frequency::dictionary::fetch())
                        .order_by(model::frequency::dictionary::order(vec![
                            model::dictionary::rank::order(SortOrder::Asc),
                        ]))
                        .exec();
                    let accents = db
                        .client()
                        .pitch_accent()
                        .find_many(vec![
                            model::pitch_accent::word::equals(word.clone()),
                            model::pitch_accent::reading::equals(reading.clone()),
                        ])
                        .with(model::pitch_accent::dictionary::fetch())
                        .order_by(model::pitch_accent::dictionary::order(vec![
                            model::dictionary::rank::order(SortOrder::Asc),
                        ]))
                        .exec();

                    (entry, tokio::join!(frequencies, accents))
                })
            })
            .collect::<Vec<_>>();

        let mut results = Vec::with_capacity(words.len());

        for handle in words {
            results.push(handle.await.unwrap());
        }

        let mut definitions = results
            .into_iter()
            .map(|result| {
                let (mut entry, (frequencies, accents)) = result;

                let frequencies = frequencies.unwrap_or_else(|_| vec![]);
                let accents = accents.unwrap_or_else(|_| vec![]);

                entry.frequencies.append(
                    &mut frequencies
                        .into_iter()
                        .map(|freq| Freq {
                            id: Muid::try_from_slice(&freq.id).unwrap().to_string(),
                            dictionary: freq.dictionary.unwrap().title,
                            freq: freq.display.unwrap_or(freq.frequency.to_string()),
                        })
                        .collect::<Vec<_>>(),
                );

                entry.accents.append(
                    &mut accents
                        .into_iter()
                        .map(|accent| Accent {
                            id: Muid::try_from_slice(&accent.id).unwrap().to_string(),
                            dictionary: accent.dictionary.unwrap().title,
                            reading: accent.reading,
                            accent: accent.position as u8,
                        })
                        .collect::<Vec<_>>(),
                );

                entry
            })
            .collect::<Vec<_>>();

        definitions.sort_by(|a, b| {
            let a = a
                .frequencies
                .iter()
                .map(|it| u32::from_str_radix(&it.freq, 10).unwrap_or(999999))
                .min()
                .unwrap_or(999999);
            let b = b
                .frequencies
                .iter()
                .map(|it| u32::from_str_radix(&it.freq, 10).unwrap_or(999999))
                .min()
                .unwrap_or(999999);

            a.cmp(&b)
        });

        Ok(Self::Res { definitions })
    }
}
