use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use features::shared::infra::database::Sqlite;
use language_pack::jp::get_reading_from_anki;
use prisma_client::model;
use railgun::error::Error;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;
use srs_bridge_anki::client::AnkiClient;
use srs_bridge_anki::client::actions::card::CardsInfoRequest;
use srs_bridge_anki::client::actions::card::FindCardsRequest;
use srs_bridge_anki::client::actions::deck::DeckNamesAndIdsRequest;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct KnownWordsRequest {}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct KnownWordsResponse {
    words: Vec<KnownWord>,
}

#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct KnownWord {
    word: String,
    reading: String,
    anki: bool,
    freq: Option<i32>,
}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
#[derive(Error)]
#[error(module)]
pub enum SerDeError {
    Placeholder {},
}

impl TryInto<()> for KnownWordsRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<(), Self::Error> {
        todo!();
    }
}

impl TryFrom<()> for KnownWordsResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ()) -> Result<Self, Self::Error> {
        todo!();
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
#[api(format = "json")]
#[serde(untagged, rename = "KnownWordsError")]
pub enum ApiError {
    #[api(status = "INTERNAL_SERVER_ERROR", code = "")]
    Unknown(ApiErrorKind<Option<()>>),
}

impl From<core::convert::Infallible> for ApiError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct KnownWordsState {
    db: Arc<Sqlite>,
}

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(state): State<KnownWordsState>,
    Json(body): Json<KnownWordsRequest>,
) -> ApiResult<KnownWordsResponse, ApiError> {
    let client = AnkiClient::default();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let deck_info = client.request(DeckNamesAndIdsRequest {}).await.unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("{:#?}", end - start);
    let card_info = client
        .request(FindCardsRequest {
            query: "deck:\"Refold JP1K v2\"".into(),
        })
        .await
        .unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("{:#?}", end - start);
    let card_data = client
        .request(CardsInfoRequest { cards: card_info })
        .await
        .unwrap();

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("{:#?}", end - start);

    {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let card_info = client
            .request(FindCardsRequest {
                query: "\"deck:Refold JP1K v2\" is:due".into(),
            })
            .await
            .unwrap();

        // println!("{card_info:#?}");

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        // println!("{:#?}", end - start);
    };

    // println!("{:#?}", card_data.iter().map(|it| it.due).collect::<Vec<_>>());

    let anki_cards = card_data[0..]
        .to_vec()
        .into_iter()
        .map(|it| {
            (
                true,
                it.fields.get("Word").unwrap().value.clone(),
                it.fields
                    .get("Word With Reading")
                    .map(|it| it.value.clone())
                    .unwrap_or_else(|| {
                        it.fields
                            .get("WordReading")
                            .map(|it| it.value.clone())
                            .unwrap_or_default()
                    }),
            )
        })
        .collect::<Vec<_>>();

    let mut cards = state
        .db
        .client()
        .card()
        .find_many(vec![])
        .exec()
        .await
        .unwrap()
        .into_iter()
        .map(|it| (false, it.word, it.reading))
        .collect::<Vec<_>>();

    cards.extend_from_slice(&anki_cards);

    let mut freq_map: HashMap<String, i32> = HashMap::default();
    let mut known_words: Vec<KnownWord> = vec![];

    for card in cards {
        let (is_anki, word, reading) = card;

        if !freq_map.contains_key(&word) {
            let freqs = state
                .db
                .client()
                .frequency()
                .find_many(vec![model::frequency::word::equals(word.clone())])
                .exec()
                .await
                .unwrap();
            let freq = freqs.iter().fold(9999999, |a, b| b.frequency.min(a));

            freq_map.insert(word.clone(), freq);
        }

        known_words.push(KnownWord {
            word: word.clone(),
            reading: get_reading_from_anki(reading.clone()),
            anki: is_anki,
            freq: Some(*freq_map.get(&word).unwrap()),
        });
    }

    ApiResponse::success(StatusCode::OK, KnownWordsResponse { words: known_words })
}
