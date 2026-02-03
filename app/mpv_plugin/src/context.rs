use std::sync::Arc;

use arboard::Clipboard;
use mpv_client::Client;
use tokio::sync::Mutex;

pub struct InnerContext {
    clipboard: Clipboard,
    subtitles: Vec<Subtitle>,
    pub client: Client,
}

#[derive(Clone)]
pub struct MpvContext(Arc<Mutex<InnerContext>>);

#[derive(Clone)]
pub struct Subtitle {
    pub ts_0: f64,
    pub ts_1: f64,
    pub text: String,
}

impl MpvContext {
    pub fn new(client: Client) -> Self {
        Self(Arc::new(Mutex::new(InnerContext {
            clipboard: Clipboard::new().unwrap(),
            subtitles: vec![],
            client,
        })))
    }

    pub async fn get_float_property(&self, name: &str) -> mpv_client::Result<f64> {
        self.0.lock().await.client.get_property::<f64>(name)
    }

    pub async fn get_string_property(&self, name: &str) -> mpv_client::Result<String> {
        self.0.lock().await.client.get_property::<String>(name)
    }

    pub async fn clear_subtitles(&self) {
        self.0.lock().await.subtitles.clear();
    }

    pub async fn set_clipboard(&self, text: String) {
        self.0.lock().await.clipboard.set_text(text).unwrap();
    }

    pub async fn add_subtitle(&self, subtitle: Subtitle) {
        self.0.lock().await.subtitles.push(subtitle);
    }
}
