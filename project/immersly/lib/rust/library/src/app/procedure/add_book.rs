use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::fs::read_to_string;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Mul;
use std::pin::Pin;
use std::sync::Arc;

use blart::TreeMap;
use dictionary::app::service::dictionary_trie::DictionaryTrieService;
use epub::archive::EpubArchive;
use itertools::Itertools;
use language_pack::Transcription;
use language_pack::segment::TextSegmenter;
use language_pack::transform::LanguageTransformer;
use language_pack_jp::japanese_language_pipeline;
use language_pack_jp::segment::JapaneseTextSegmenter;
use language_pack_jp::transcription::EbookSegments;
use language_pack_jp::transform::JAPANESE_TRANSFORMS;
use language_pack_jp::transform::group_inflected;
use language_pack_jp::transform::transform_japanese_text;
use railgun::di::Component;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use shared::domain::value::existing_file::ExistingFile;
use shared::infra::Procedure;
use storage::app::procedure::add_resource::AddResourceProcedure;
use storage::app::procedure::add_resource::AddResourceReq;
use storage::app::procedure::commit_resource::CommitResourceProcedure;
use storage::app::procedure::commit_resource::CommitResourceReq;
use storage::app::procedure::prepare_resource::PrepareResourceProcedure;
use storage::app::procedure::prepare_resource::PrepareResourceReq;

use crate::domain::entity::book::Book;
use crate::infra::repository::book::BookRepository;

#[derive(Debug)]
pub struct AddBookReq {
    pub title: String,
    pub book_path: ExistingFile,
    pub audio_path: ExistingFile,
}

#[derive(Component)]
pub struct AddBookProcedure {
    // db: Arc<Sqlite>,
    book_repository: Arc<BookRepository>,
    add_resource: Arc<AddResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
    commit_resource: Arc<CommitResourceProcedure>,
    dictionary_tries: Arc<DictionaryTrieService>,
}

impl Procedure for AddBookProcedure {
    type Err = core::convert::Infallible;
    type Req = AddBookReq;
    type Res = ();

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let mut audio_timing_path = data.audio_path.as_path().to_owned();
        audio_timing_path.set_extension("json");

        assert!(audio_timing_path.exists(), "timing not generated");

        // Extract raw epub data
        let mut epub = EpubArchive::open(data.book_path.as_str()).unwrap();
        let text = epub.segments().unwrap();
        let mut text_data = EbookSegments::new(text);

        // Load timing data
        let timing_data = read_to_string(&audio_timing_path).unwrap();
        let timing_data: Transcription = serde_json::from_str(&timing_data).unwrap();

        let time = std::time::Instant::now();
        //
        let pipeline = japanese_language_pipeline();
        pipeline.run(&timing_data, &mut text_data);

        let adaptive = self.dictionary_tries.get();
        let adaptive_readings = self.dictionary_tries.get_readings();

        let times = text_data
            .0
            .par_iter()
            .map(|item| {
                let segments = transform_japanese_text(item.text(), &adaptive, &adaptive_readings);

                (item.time, item.kind(), segments)
            })
            .collect::<Vec<_>>();

        let serialized_data = serde_json::to_string(&times).unwrap();
        // std::fs::write("/tmp/eigjii", data);

        let title = epub.title().to_owned();

        //
        let resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: "rendered.txt".to_owned(),
            })
            .await
            .unwrap();

        std::fs::write(&resource.path, "").unwrap();

        self.commit_resource
            .run(CommitResourceReq {
                resource: resource.resource.clone(),
            })
            .await
            .unwrap();

        let audio_resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: "timing".to_owned(),
            })
            .await
            .unwrap();

        std::fs::write(&audio_resource.path, serialized_data).unwrap();

        self.commit_resource
            .run(CommitResourceReq {
                resource: audio_resource.resource,
            })
            .await
            .unwrap();

        let resource = self
            .add_resource
            .run(AddResourceReq {
                path: data.audio_path.clone(),
            })
            .await
            .unwrap();

        let audio_id = resource.id().clone();

        let book = Book::new(
            title,
            data.book_path.clone(),
            data.book_path,
            ExistingFile::from_path(audio_resource.path.into()),
            audio_id,
        );

        self.book_repository.create(&book).await;

        Ok(())
    }
}
