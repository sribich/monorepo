use std::ffi::CString;
use std::fs::File;
use std::fs::read_to_string;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

use blart::TreeMap;
use epub::archive::EpubArchive;
use itertools::Itertools;
use language_pack::segment::TextSegmenter;
use language_pack::Transcription;
use language_pack::transform::TextTransform;
use language_pack_jp::japanese_language_pipeline;
use language_pack_jp::segment::JapaneseTextSegmenter;
use language_pack_jp::transcription::EbookSegments;
use language_pack_jp::transcription::JapaneseTranscriptionContext;
use language_pack_jp::transform::group_inflected;
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
use crate::domain::entity::book::BookData;
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
}

impl Procedure for AddBookProcedure {
    type Err = core::convert::Infallible;
    type Req = AddBookReq;
    type Res = ();

    // * book_path
    // * audio_path
    //
    // Derived from `audio_path`:
    //
    //   * audio_timing.json
    //
    // Outputs:
    //
    //  * rendered.json
    //    * text
    //    * freq
    //    * timestamp
    //    * segments
    //      * joined / broken down segments for narrowing. (word / base)
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

        //
        // let pipeline = japanese_language_pipeline();
        // pipeline.run(&timing_data, &text_data);
        // panic!();

        // Timestamp segments
        let transcriber = JapaneseTranscriptionContext {};

        transcriber.test(&mut text_data, &timing_data);

        //
        //
        //
        let mut adaptive = TreeMap::<CString, Option<usize>>::new();
        let mut adaptive_readings = TreeMap::<CString, Option<usize>>::new();

        let lines = BufReader::new(File::open("/home/nulliel/Result_45.csv").unwrap()).lines();
        for line in lines.map_while(Result::ok) {
            if line == r#""""# {
                continue;
            }

            if let Some((left, right)) = line.split_once(",") {
                if let Some((middle, right)) = right.split_once(",") {
                    let freq = usize::from_str_radix(right, 10).ok();

                    if left != r#""""# {
                        adaptive.insert(CString::new(left).unwrap(), freq);
                    }

                    if middle != r#""""# {
                        adaptive_readings.insert(CString::new(middle).unwrap(), freq);
                    }
                }
            }
        }

        let times = text_data
            .0
            .par_iter()
            .map(|item| {
                let segments = self.get_segments(item.text(), &adaptive, &adaptive_readings);

                return (item.time, item.kind(), segments);
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

        /*

        let title = result.package.metadata.title.first().unwrap().value.clone();
        let rendered = result.rendered;

        // Load audio path and timing information
        let timing_data = read_to_string(data.audio_path.as_path().with_extension("json")).unwrap();

        let existing_file = ExistingFile::from_path(data.audio_path.as_path().to_owned());








         */

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

impl AddBookProcedure {
    fn get_segments(
        &self,
        line: &str,
        adaptive: &TreeMap<CString, Option<usize>>,
        adaptive_readings: &TreeMap<CString, Option<usize>>,
    ) -> Vec<(String, String, Option<usize>)> {
        let segmenter = JapaneseTextSegmenter::new();
        let segments = group_inflected(segmenter.segment(line));

        let mut output = vec![];

        for (segment, inflects) in &segments {
            if *inflects {
                let mut transform = TextTransform::new(segment.to_owned());

                let resolve = transform.resolve(&adaptive);

                if resolve.is_empty() {
                    output.push((segment.clone(), segment.clone()));
                } else {
                    let mut out = resolve
                        .into_iter()
                        .map(|it| {
                            let base = it.1;

                            let inflected = it.0.map_or_else(
                                || base.clone(),
                                |inflection| {
                                    if inflection.last_inflection == "" {
                                        return base.clone();
                                    }

                                    if inflection.last_inflection.len() == base.len() {
                                        return inflection.inflection;
                                    }

                                    return [
                                        &base[..(base.len().saturating_sub(inflection.last_inflection.len()))],
                                        &inflection.inflection,
                                    ]
                                    .join("");
                                },
                            );

                            (inflected, base)
                        })
                        .collect::<Vec<_>>();

                    output.append(&mut out);
                }
            } else {

                output.push((segment.clone(), segment.clone()));
            }
        }


        let mut result = vec![];
        let mut i = 0;

        while i < output.len() {
            let segment = &output[i];

            if i == output.len() - 1 {
                result.push(segment.clone());
                i += 1;
                continue;
            }

            if !adaptive.contains_key(&CString::new(segment.0.as_bytes()).unwrap()) {
                result.push(segment.clone());
                i += 1;
                continue;
            }

            let mut key_with_boundary = i + 1;
            let mut curr_check = i + 2;

            while curr_check <= output.len()
                && let Some(prefix) = adaptive.get_prefix(
                    &CString::new(
                        output[i..curr_check]
                            .iter()
                            .map(|it| &it.0)
                            .join("")
                            .as_bytes(),
                    )
                    .unwrap(),
                )
            {
                if adaptive.contains_key(
                    &CString::new(
                        output[i..curr_check]
                            .iter()
                            .map(|it| &it.0)
                            .join("")
                            .as_bytes(),
                    )
                    .unwrap(),
                ) {
                    key_with_boundary = curr_check;
                }

                curr_check += 1;
            }

            let entry = &output[i..key_with_boundary];
            i = key_with_boundary;

            // println!("{i} {key_with_boundary} {curr_check} {entry:#?}");

            if entry.len() == 1 {
                result.push(segment.clone());
            } else {
                result.push(
                    entry
                        .iter()
                        .fold((String::new(), String::new()), |prev, curr| {
                            let a = format!("{}{}", prev.0, curr.0);

                            (a.clone(), a)
                        }),
                );
            }
        }

        let result = result
            .into_iter()
            .map(|it| {
                let freq = adaptive
                    .get(&CString::new(it.0.as_bytes()).unwrap())
                    .map(|it| *it)
                    .or_else(|| {
                        adaptive_readings
                            .get(&CString::new(it.0.as_bytes()).unwrap())
                            .map(|it| *it)
                            .or(None)
                    })
                    .flatten();

                (it.0, it.1, freq)
            })
            .collect::<Vec<_>>();

        result
    }
}
