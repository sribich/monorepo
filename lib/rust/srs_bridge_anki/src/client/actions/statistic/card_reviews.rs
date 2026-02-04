use serde::Deserialize;
use serde::Serialize;

use crate::client::AnkiRequest;

#[derive(Default, Debug, Clone, Serialize)]
pub struct CardReviewsRequest {
    pub deck: String,
    #[serde(rename = "startID")]
    pub start_id: usize,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(
    expecting = "expecting [<review_time>, <card_id>, <usn>, <button_pressed>, <new_interval>, <previous_interval>, <new_factor>, <review_duration>, <review_type>]"
)]
pub struct CardReviewsResponse {
    /// The time the card was reviewed.
    pub review_time: usize,
    ///
    pub card_id: usize,
    ///
    pub usn: i64,
    ///
    pub button_pressed: usize,
    ///
    pub new_interval: i64,
    ///
    pub previous_interval: i64,
    ///
    pub new_factor: usize,
    ///
    pub review_duration: usize,
    ///
    pub review_type: usize,
}

impl AnkiRequest for CardReviewsRequest {
    type Response = Vec<CardReviewsResponse>;

    const ACTION: &'static str = "cardReviews";
    const VERSION: u8 = 6;
}
