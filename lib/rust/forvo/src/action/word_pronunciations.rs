use serde::Deserialize;
use serde::Serialize;

use crate::client::ForvoRequest;

#[derive(Debug, Serialize)]
pub struct WordPronunciationsRequest {
    pub word: String,
    /// Language code from <https://forvo.com/languages-codes/>
    ///
    /// TODO: Enumify ^
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Country code (alpha-3) <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3>
    ///
    /// TODO: Enumify ^
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<u32>,
    /// Use rate-desc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "kebab-case")]
    pub group_in_languages: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WordPronunciationsResponse {
    pub attributes: Attributes,
    pub items: Vec<WordPronunciation>,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct WordPronunciation {
    pub id: u32,
    pub word: String,
    pub original: String,
    pub hits: u32,
    pub username: String,
    pub sex: String,
    pub country: String,
    pub code: String,
    pub langname: String,
    pub pathmp3: String,
    pub pathogg: String,
    pub rate: i32,
    pub num_votes: u32,
    pub num_positive_votes: u32,
}

impl ForvoRequest for WordPronunciationsRequest {
    type Response = WordPronunciationsResponse;

    const ACTION: &'static str = "word-pronunciations";
    const FORMAT: &'static str = "json";
}
