use std::sync::Arc;

use language_pack::jp::create_reading;
use prisma_client::model::SortOrder;
use prisma_client::model::{self};
use railgun::typegen::Typegen;
use railgun::di::Component;
use serde::Deserialize;
use serde::Serialize;
use shared::domain::value::muid::Muid;
use shared::infra::Procedure;
use shared::infra::database::Sqlite;

#[derive(Debug, Deserialize, Typegen)]
pub struct GetExactWordReq {
    pub word: String,
    pub reading: Option<String>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct GetExactWordRes {
    word: DictionaryWord,
}

#[derive(Debug, Serialize, Typegen)]
pub struct DictionaryWord {
    word: String,
    reading: Option<String>,
    reading_ruby: Option<String>,

    definitions: Vec<Definition>,
    bilingual_definition: Option<Definition>,

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
pub struct GetExactWordProcedure {
    db: Arc<Sqlite>,
}

impl Procedure for GetExactWordProcedure {
    type Err = core::convert::Infallible;
    type Req = GetExactWordReq;
    type Res = GetExactWordRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let client = self.db.client();
        let db = Arc::clone(&self.db);

        let words = client
            .word()
            .find_many(vec![
                model::word::word::equals(data.word.clone()),
                model::word::reading::equals(data.reading.clone().unwrap_or_default()),
            ])
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await
            .unwrap();

        let bilingual_definition = db
            .client()
            .word()
            .find_first(vec![
                model::word::word::equals(data.word.clone()),
                model::word::reading::equals(data.reading.clone().unwrap_or_default()),
                model::word::dictionary::is(vec![model::dictionary::language_type::equals(
                    "bi".to_owned(),
                )]),
            ])
            .with(model::word::dictionary::fetch())
            .order_by(model::word::dictionary::order(vec![
                model::dictionary::rank::order(SortOrder::Asc),
            ]))
            .exec()
            .await
            .unwrap()
            .map(|word| Definition {
                id: Muid::try_from_slice(&word.id).unwrap().to_string(),
                dictionary: word.dictionary.as_ref().unwrap().title.clone(),
                definition: word.definition.clone(),
            });

        let mut dictionary_word = DictionaryWord {
            word: data.word.clone(),
            reading: data.reading.clone(),
            reading_ruby: data.reading.map(|it| create_reading(&data.word, &it)),
            definitions: vec![],
            bilingual_definition,
            frequencies: vec![],
            accents: vec![],
        };

        for word in &words {
            dictionary_word.definitions.push(Definition {
                id: Muid::try_from_slice(&word.id).unwrap().to_string(),
                dictionary: word.dictionary.as_ref().unwrap().title.clone(),
                definition: word.definition.clone(),
            });
        }

        let word = dictionary_word.word.clone();
        let reading = dictionary_word.reading.clone().unwrap_or_default();

        let result = tokio::spawn(async move {
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

            tokio::join!(frequencies, accents)
        });

        let (frequencies, accents) = result.await.unwrap();

        let frequencies = frequencies.unwrap_or_else(|_| vec![]);
        let accents = accents.unwrap_or_else(|_| vec![]);

        dictionary_word.frequencies.append(
            &mut frequencies
                .into_iter()
                .map(|freq| Freq {
                    id: Muid::try_from_slice(&freq.id).unwrap().to_string(),
                    dictionary: freq.dictionary.unwrap().title,
                    freq: freq.display.unwrap_or(freq.frequency.to_string()),
                })
                .collect::<Vec<_>>(),
        );

        dictionary_word.accents.append(
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

        Ok(Self::Res {
            word: dictionary_word,
        })
    }
}

/*

*/
