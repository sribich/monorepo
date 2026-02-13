use forvo::action::word_pronunciations::WordPronunciationsRequest;
use forvo::action::word_pronunciations::WordPronunciationsResponse;
use forvo::client::ForvoClient;

pub struct ForvoFetcher {}

impl ForvoFetcher {
    pub async fn get_pronunciations(&self, word: String) -> WordPronunciationsResponse {
        let key = std::env::var("FORVO_API_KEY").unwrap();

        let client = ForvoClient::new(key.clone());

        client
            .request(WordPronunciationsRequest {
                word: word.clone(),
                language: Some("ja".to_owned()),
                country: Some("jpn".to_owned()),
                username: None,
                sex: None,
                rate: None,
                order: None,
                limit: None,
                group_in_languages: None,
            })
            .await
            .unwrap()
    }
}
