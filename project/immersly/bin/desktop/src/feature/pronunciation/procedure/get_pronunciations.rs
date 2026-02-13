use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use railgun::typegen::Typegen;
use railgun_di::Component;
use serde::Serialize;

use crate::feature::pronunciation::service::pronunciation::PronunciationService;
use features::shared::infra::Procedure;

//==============================================================================
// Data
//==============================================================================
pub struct Req {
    pub word: String,
    // pub reading: String,
    pub speaker_id: Option<Muid>,
}

pub struct Res {
    pub pronunciations: Vec<Pronunciation>,
}

#[derive(Debug, Serialize, Typegen)]
pub struct Pronunciation {
    id: String,
    speaker: String,
    sex: String,
}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct GetPronunciationsProcedure {
    pronunciation_service: Arc<PronunciationService>,
}

impl Procedure for GetPronunciationsProcedure {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let Req {
            word,
            // reading,
            speaker_id,
        } = data;

        let needs_pronunciations = self
            .pronunciation_service
            .needs_pronunciations(word.clone())
            .await;

        if needs_pronunciations {
            self.pronunciation_service.fetch_pronunciations(&word).await;
        }

        let mut pronunciations = self.pronunciation_service.list_pronunciations(word).await;

        if speaker_id.is_some() {
            pronunciations = pronunciations
                .into_iter()
                .filter(|it| it.id == speaker_id.as_ref().unwrap().as_bytes())
                .collect::<Vec<_>>();
        }

        Ok(Res {
            pronunciations: pronunciations
                .into_iter()
                .map(|it| Pronunciation {
                    id: Muid::try_from_slice(&it.id).unwrap().to_string(),
                    speaker: it.name,
                    sex: it.sex,
                })
                .collect::<Vec<_>>(),
        })
    }
}
