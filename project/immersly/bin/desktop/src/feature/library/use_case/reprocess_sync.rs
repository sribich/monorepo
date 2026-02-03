use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use epub::archive::EpubArchive;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use language_pack::jp::transcription::JapaneseTranscriptionContext;
use language_pack::jp::transcription::SegmentKind;
use prisma_client::model;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;
use tokio::fs::File;
use tokio::fs::read_to_string;
use tokio::io::AsyncWriteExt;

use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::settings::domain::value::setting::data_path::DataPath;
use crate::system::UseCase;

#[derive(Debug)]
pub struct ReprocessSyncData {
    pub book_id: Muid,
}

#[derive(Component)]
pub struct ReprocessSyncUseCase {
    db: Arc<Sqlite>,
    setting_service: Arc<SettingService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampedSegments {
    pub t0: Option<u64>,
    pub t1: Option<u64>,
    pub kind: SegmentKind,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub word: String,
    pub base: String,
    pub freq: Option<i32>,
}

impl UseCase for ReprocessSyncUseCase {
    type Err = core::convert::Infallible;
    type Req = ReprocessSyncData;
    type Res = ();

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        // book_id is the media_id right now. Need to fix.

        let book = self
            .db
            .client()
            .book()
            .find_unique(model::book::id::equals(data.book_id.as_bytes().to_vec()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        let result = EpubArchive::open(book.path.as_str()).unwrap();
        let rendered_data = result.chapters;

        let data_path = self.setting_service.get_setting::<DataPath>().await;
        let mut data_path = PathBuf::from(data_path.to_string());

        data_path.push(data.book_id.to_string());

        let timing_data = read_to_string(data_path.join("audio_timing.json"))
            .await
            .unwrap();

        let transcriber = JapaneseTranscriptionContext {};

        let fit_data = transcriber.fit_new(rendered_data, timing_data);
        let mut new_data: Vec<TimestampedSegments> = vec![];

        let mut freq_map: HashMap<String, i32> = HashMap::default();

        for data in fit_data {
            let mut segments = vec![];

            for segment in data.segments {
                if segment.freq && !freq_map.contains_key(&segment.base) {
                    let freqs = self
                        .db
                        .client()
                        .frequency()
                        .find_many(vec![model::frequency::word::equals(segment.base.clone())])
                        .exec()
                        .await
                        .unwrap();
                    let freq = freqs.iter().fold(9999999, |a, b| b.frequency.min(a));

                    freq_map.insert(segment.base.clone(), freq);
                }

                segments.push(Segment {
                    word: segment.word,
                    freq: segment.freq.then(|| *freq_map.get(&segment.base).unwrap()),
                    base: segment.base,
                });
            }

            new_data.push(TimestampedSegments {
                t0: data.t0,
                t1: data.t1,
                kind: data.kind,
                segments,
            });
        }

        let fit_data = serde_json::to_string(&new_data).unwrap();

        let fit_path = data_path.join("rendered_timing.txt");

        let mut file = File::create(fit_path).await.unwrap();

        file.write_all(fit_data.as_bytes()).await.unwrap();

        /*


        /*
        let result = EpubArchive::open(data.path.as_str()).unwrap();

        let title = result.package.metadata.title.first().unwrap().value.clone();
        let rendered = result.rendered;

        let mut media = Media::new(title, data.series_id);

        let base_data_path = self.setting_service.get_setting::<DataPath>().await;
        let fs = Fs::new(base_data_path.path());

        let rendered_path = fs
            .write(format!("{}/rendered.txt", media.id().to_string()), rendered)
            .await;

        let book = Book::new(data.path, ExistingPath::from_path(rendered_path));

        media.set_book(book);

        self.library_repository.writer().save_media(media).await;
         */
         */

        Ok(())
    }
}
