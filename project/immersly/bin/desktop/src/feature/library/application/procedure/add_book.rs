use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Arc;

use epub::archive::EpubArchive;
use features::shared::infra::database::Sqlite;
use language_pack::jp::transcription::JapaneseTranscriptionContext;
use prisma_client::model;
use railgun_di::Component;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::library::domain::aggregate::book::Book;
use crate::feature::library::domain::aggregate::media::Media;
use crate::feature::library::domain::repository::book::BookRepository;
use crate::feature::library::use_case::reprocess_sync::Segment;
use crate::feature::library::use_case::reprocess_sync::TimestampedSegments;
use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::settings::domain::value::setting::data_path::DataPath;
use crate::feature::storage::procedure::add_resource::AddResourceProcedure;
use crate::feature::storage::procedure::add_resource::AddResourceReq;
use crate::feature::storage::procedure::commit_resource::CommitResourceProcedure;
use crate::feature::storage::procedure::commit_resource::CommitResourceReq;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceProcedure;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceReq;
use crate::system::Procedure;
use crate::system::fs::Fs;

#[derive(Debug)]
pub struct AddBookData {
    pub title: String,
    pub book_path: ExistingPath,
    pub audio_path: ExistingPath,
}

#[derive(Component)]
pub struct AddBookProcedure {
    db: Arc<Sqlite>,
    book_repository: Arc<dyn BookRepository>,
    add_resource: Arc<AddResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
    commit_resource: Arc<CommitResourceProcedure>,
}

impl Procedure for AddBookProcedure {
    type Err = core::convert::Infallible;
    type Req = AddBookData;
    type Res = ();

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let result = EpubArchive::open(data.book_path.as_str()).unwrap();

        let title = result.package.metadata.title.first().unwrap().value.clone();
        let rendered = result.rendered;

        let resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: "rendered.txt".to_owned(),
            })
            .await
            .unwrap();

        std::fs::write(&resource.path, rendered.clone()).unwrap();

        self.commit_resource
            .run(CommitResourceReq {
                resource: resource.resource.clone(),
            })
            .await
            .unwrap();

        // Load audio path and timing information
        let timing_data = read_to_string(data.audio_path.as_path().with_extension("json")).unwrap();

        let audio_id = self
            .add_resource
            .run(AddResourceReq {
                path: data.audio_path.clone(),
            })
            .await
            .unwrap();

        println!("{:#?}", result.chapters[0]);

        let transcriber = JapaneseTranscriptionContext {};
        let fit_data = transcriber.fit_new(result.chapters, timing_data);

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

        let audio_resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: "timing".to_owned(),
            })
            .await
            .unwrap();

        std::fs::write(&audio_resource.path, fit_data);

        self.commit_resource
            .run(CommitResourceReq {
                resource: audio_resource.resource,
            })
            .await
            .unwrap();

        let book = Book::new(
            title,
            data.book_path,
            ExistingPath::new(resource.path.clone()).unwrap(),
            ExistingPath::new(audio_resource.path.clone()).unwrap(),
            audio_id,
        );

        self.book_repository.writer().create(&book).await;

        Ok(())
    }
}

/*
let data_path = self.setting_service.get_setting::<DataPath>().await;
        let mut data_path = PathBuf::from(data_path.to_string());

        data_path.push(data.media_id.to_string());

        let rendered_data = read_to_string(data_path.join("rendered.txt"))
            .await
            .unwrap();

        let timing_data = read_to_string(data_path.join("audio_timing.json"))
            .await
            .unwrap();

        let transcriber = JapaneseTranscriptionContext {};

        let fit_data = transcriber.fit(rendered_data, timing_data);
        let fit_path = data_path.join("rendered_timing.txt");

        let mut file = File::create(fit_path).await.unwrap();

        file.write_all(fit_data.as_bytes()).await.unwrap();
*/


/*
OLD ADD_AUDIO


        let data_path = self.setting_service.get_setting::<DataPath>().await;
        let mut data_path = PathBuf::from(data_path.to_string());

        data_path.push(data.media_id.to_string());

        let rendered_data = read_to_string(data_path.join("rendered.txt"))
            .await
            .unwrap();

        let timing_data = read_to_string(data_path.join("audio_timing.json"))
            .await
            .unwrap();

        let transcriber = JapaneseTranscriptionContext {};

        let fit_data = transcriber.fit(rendered_data, timing_data);
        let fit_path = data_path.join("rendered_timing.txt");

        let mut file = File::create(fit_path).await.unwrap();

        file.write_all(fit_data.as_bytes()).await.unwrap();

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
        println!("here");
        Ok(())
*/
