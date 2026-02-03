use std::path::PathBuf;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use language_pack::jp::transcription::JapaneseTranscriptionContext;
use railgun_di::Component;
use tokio::fs::File;
use tokio::fs::read_to_string;
use tokio::io::AsyncWriteExt;

use crate::domain::value::existing_path::ExistingPath;
use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::settings::domain::value::setting::data_path::DataPath;
use crate::system::UseCase;

pub struct AddAudioData {
    pub media_id: Muid,
    pub path: ExistingPath,
}

#[derive(Component)]
pub struct AddAudioUseCase {
    setting_service: Arc<SettingService>,
}

impl UseCase for AddAudioUseCase {
    type Err = core::convert::Infallible;
    type Req = AddAudioData;
    type Res = ();

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
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
    }
}
