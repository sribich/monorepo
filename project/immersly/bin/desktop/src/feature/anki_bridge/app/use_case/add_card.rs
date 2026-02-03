use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use itertools::Itertools;
use railgun_di::Component;

use crate::system::UseCase;

//==============================================================================
// Data
//==============================================================================
pub struct Req {
    pub media_id: Muid,
    pub audio_id: Muid,
    pub word: String,
    pub reading: String,
    pub sentence: String,
    pub sentence_timestamp: (i64, i64),
}

pub struct Res {}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct AddCardUseCase {
    // db: Arc<Sqlite>,
    // anki_client: Arc<AnkiService>,
    // dictionary_lookup: Arc<DictionaryLookupService>,
    // definitions_data_view: Arc<DefinitionsDataView>,
    // setting_service: Arc<SettingService>,
    // anki_media: Arc<AnkiMediaService>,
}

impl UseCase for AddCardUseCase {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        /*
        let book = self
            .db
            .client()
            .book()
            .find_unique(model::book::media_id::equals(
                data.media_id.as_bytes().to_vec(),
            ))
            .exec()
            .await
            .unwrap()
            .unwrap();

        let definitions = self
            .definitions_data_view
            .from_word(data.word.clone(), Some(data.reading.clone()))
            .await;

        let pronunciation = self
            .db
            .client()
            .pronunciation()
            .find_unique(model::pronunciation::id::equals(
                data.audio_id.as_bytes().to_vec(),
            ))
            .exec()
            .await
            .unwrap()
            .unwrap();

        println!("{:#?}", data.audio_id.to_string());
        println!("{:#?}", pronunciation.path);

        let word_audio_path = self
            .anki_media
            .add_media_file(data.audio_id.to_string(), pronunciation.path)
            .await;

        let mut data_path = self
            .setting_service
            .get_setting::<DataPath>()
            .await
            .to_path();

        data_path.push("books");
        data_path.push(data.media_id.to_string());
        data_path.push("clips");
        let uuid = Uuid::new_v4().simple().to_string();
        let sentence_audio = data_path.join(&uuid).join("sentence.mp3");
        create_dir_all(data_path.join(&uuid)).unwrap();

        //       let x = ffmpeg_sidecar::command::FfmpegCommand::new()
        //           .seek(format!("{}ms", data.sentence_timestamp.0))
        //           .input(book.audio_path.unwrap())
        //           .duration(format!(
        //               "{}ms",
        //               data.sentence_timestamp.1 - data.sentence_timestamp.0
        //           ))
        //           .output(&sentence_audio.to_str().unwrap().to_string())
        //           .spawn()
        //           .unwrap();

        // .iter()
        // .unwrap()
        // .for_each(|event| {
        // println!("{:#?}", event);
        // });
        // x.wait();

        let sentence_audio_path = self
            .anki_media
            .add_media_file(uuid.clone(), sentence_audio.to_str().unwrap().to_string())
            .await;

        let fields = [
            ("Key", &data.word[..]),
            ("Word", &data.word[..]),
            (
                "WordReading",
                &create_reading(&data.word, &data.reading)[..],
            ),
            (
                "PrimaryDefinition",
                &definitions.monolingual.first().unwrap().definition[..],
            ),
            (
                "SecondaryDefinition",
                &definitions.bilingual.first().unwrap().definition[..],
            ),
            (
                "ExtraDefinitions",
                &definitions
                    .monolingual
                    .iter()
                    .skip(1)
                    .map(|it| &it.definition)
                    .join("\n\n")[..],
            ),
            ("WordAudio", &format!("[sound:{}]", word_audio_path)[..]),
            ("Sentence", &data.sentence[..]),
            (
                "SentenceAudio",
                &format!("[sound:{}]", sentence_audio_path)[..],
            ),
        ]
        .into_iter()
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .collect::<HashMap<String, String>>();

        self.anki_client
            .client()
            .request(AddNoteRequest {
                note: Note {
                    deck_name: "Refold JP1K v2".to_owned(),
                    model_name: "JP Mining Note".to_owned(),
                    fields,
                    tags: vec![],
                },
            })
            .await
            .unwrap();
         */

        Ok(Res {})
    }
}

// Sentence
// SentenceReading
// Picture
// WordAudio
// SentenceAudio
// WordReadingHiragana
// FrequenciesStylized
// FrequencySort
