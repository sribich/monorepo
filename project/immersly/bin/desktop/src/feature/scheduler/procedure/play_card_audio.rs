use core::result::Result;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use prisma_client::model;
use railgun_di::Component;

use crate::feature::scheduler::handler::play_card_audio::AudioKind;
use crate::system::Procedure;

pub struct Req {
    pub card_id: Muid,
    pub kind: AudioKind,
}

#[derive(Component)]
pub struct PlayCardAudioProcedure {
    db: Arc<Sqlite>,
}

impl Procedure for PlayCardAudioProcedure {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Option<String>;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let card = self
            .db
            .client()
            .card()
            .find_unique(model::card::id::equals(data.card_id.as_bytes().to_vec()))
            .with(model::card::reading_audio::fetch())
            .with(model::card::sentence_audio::fetch())
            .with(model::card::image::fetch())
            .exec()
            .await
            .unwrap()
            .unwrap();

        let path = match data.kind {
            AudioKind::Reading => card.reading_audio.unwrap().map(|it| it.path),
            AudioKind::Sentence => card.sentence_audio.unwrap().map(|it| it.path),
            AudioKind::Image => card.image.unwrap().map(|it| it.path),
        };

        Ok(path)
    }
}
