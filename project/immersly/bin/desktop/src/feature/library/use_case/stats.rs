use features::shared::domain::value::muid::Muid;
use railgun_di::Component;

use crate::system::UseCase;

#[derive(Component)]
pub struct StatsUseCase {}

impl UseCase for StatsUseCase {
    type Err = core::convert::Infallible;
    type Req = Muid;
    type Res = ();

    async fn run(&self, _data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        /*
        let result = EpubArchive::open(data.path.as_str()).unwrap();

        let title = result.package.metadata.title.first().unwrap().value.clone();
        let rendered = result.rendered;

        let mut media = Media::new(title, data.series_id);

        let base_data_path = self.setting_service.get_setting::<DataPath>().await;
        let fs = Fs::new(base_data_path.to_string());

        let rendered_path = fs
            .write(format!("{}/rendered.txt", media.id().to_string()), rendered)
            .await;

        let book = Book::new(data.path, ExistingPath::from_path(rendered_path));

        media.set_book(book);

        self.library_repository.writer().save_media(media).await;
         */

        Ok(())
    }
}

/*

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let book = self
            .db
            .client()
            .book()
            .find_unique(model::book::media_id::equals(data.as_bytes().to_vec()))
            .with(model::book::media::fetch().with(model::media::progress::fetch()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        let progress = if let Some(Some(progress)) = book.media.unwrap().progress {
            Some(progress.timestamp)
        } else {
            None
        };

        let data = read_to_string(book.rendered_audio_path.unwrap()).unwrap();

        Ok(Self::Res {
            text: data,
            audio_id: Muid::from_slice(&book.audio_resource_id.unwrap()).unwrap().to_string(),
            progress,
        })
    }
}
*/
