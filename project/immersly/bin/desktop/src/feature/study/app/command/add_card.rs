use std::{fs::create_dir_all, sync::Arc};

use prisma_client::model;
use railgun::typegen::Typegen;
use railgun_di::Dep;
use serde::{Deserialize, Serialize};

use crate::{
    context::settings::{
        app::service::settings::SettingService, domain::value::setting::data_path::DataPath,
    },
    domain::common::value::muid::Muid,
    shared::Command,
};

#[derive(Debug)]
pub struct AddCardCommandRequest {
    pub book_id: Muid,
    pub word: String,
    pub reading: Option<String>,
    pub reading_timestamp: (u32, u32),
    pub sentence: String,
    pub sentence_timestamp: (u32, u32),
}

#[derive(Debug, Serialize, Typegen)]
pub struct AddCardCommandResponse {}

pub struct AddCardCommand {
    setting_service: Arc<SettingService>,
    db: Arc<Sqlite>,
}

impl AddCardCommand {
    pub fn new(db: Dep<Sqlite>, setting_service: Dep<SettingService>) -> Self {
        Self {
            db: db.get(),
            setting_service: setting_service.get(),
        }
    }
}

impl Command for AddCardCommand {
    type Err = core::convert::Infallible;
    type Req = AddCardCommandRequest;
    type Res = AddCardCommandResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let card_id = Muid::new_now();

        let mut data_path = self
            .setting_service
            .get_setting::<DataPath>()
            .await
            .to_path();
        data_path.push("books");
        data_path.push(data.book_id.to_string());
        data_path.push("clips");

        let word_audio = data_path.join(card_id.to_string()).join("word.mp3");
        let sentence_audio = data_path.join(card_id.to_string()).join("sentence.mp3");

        create_dir_all(data_path.join(card_id.to_string())).unwrap();

        let x = ffmpeg_sidecar::command::FfmpegCommand::new()
            .seek(format!("{}ms", data.reading_timestamp.0))
            .input("~/Japanese/bookworm.wav")
            .duration(format!(
                "{}ms",
                data.reading_timestamp.1 - data.reading_timestamp.0
            ))
            .output(word_audio.to_str().unwrap())
            .spawn()
            .unwrap()
            .iter()
            .unwrap()
            .for_each(|event| {
                println!("{:#?}", event);
            });

        ffmpeg_sidecar::command::FfmpegCommand::new()
            .seek(format!("{}ms", data.sentence_timestamp.0))
            .input("~/Japanese/bookworm.wav")
            .duration(format!(
                "{}ms",
                data.sentence_timestamp.1 - data.sentence_timestamp.0
            ))
            .output(sentence_audio.to_str().unwrap())
            .spawn()
            .unwrap()
            .iter()
            .unwrap()
            .for_each(|event| {
                println!("{:#?}", event);
            });

        self.db
            .client()
            .card()
            .create(
                card_id.as_bytes().to_vec(),
                0,
                data.word,
                vec![
                    model::card::reading::set(data.reading),
                    model::card::reading_audio::set(Some(word_audio.to_str().unwrap().to_string())),
                    model::card::sentence_audio::set(Some(
                        sentence_audio.to_str().unwrap().to_string(),
                    )),
                    model::card::sentence::set(Some(data.sentence)),
                ],
            )
            .exec()
            .await
            .unwrap();

        Ok(Self::Res {})
    }
}

// use axum::{Json, extract::State};
// use prisma_client::model::{book, card};
// use railgun::typegen::Typegen;
// use serde::{Deserialize, Serialize};
//
// use crate::routes::AppState;
//
// pub(super) async fn add_card(
// State(AppState { db, .. }): State<AppState>,
// Json(mut data): Json<AddCardRequest>,
// ) -> Json<AddCardResponse> {
// let book = db
// .book()
// .find_unique(book::id::equals(data.book))
// .exec()
// .await
// .unwrap()
// .unwrap();
//
//
//
// println!("{:#?}", book);
//
// let result = db
// .card()
// .create(
// uuid::Uuid::now_v7().into_bytes().to_vec(),
// 0,
// data.word,
// vec![
// card::reading::set(Some(data.reading)),
// card::sentence::set(Some(data.sentence)),
// card::definition_tl::set(Some(data.definition)),
// card::definition_native::set(Some(data.definition_native)),
// ],
// )
// .exec()
// .await;
//
// Json(AddCardResponse {})
// }
//
// #[derive(Debug, Typegen, Deserialize)]
// pub(super) struct AddCardRequest {
// book: i32,
//
// word: String,
//
// reading: String,
// reading_timestamp: (i64, i64),
//
// sentence: String,
// sentence_timestamp: (i64, i64),
//
// definition: String,
// #[serde(rename = "definitionNative")]
// definition_native: String,
// }
//
// #[derive(Debug, Typegen, Serialize)]
// pub(super) struct AddCardResponse {}
//
