use std::sync::Arc;

use railgun::di::Component;
use railgun::typegen::Typegen;
use serde::Serialize;
use shared::infra::Procedure;

use crate::app::service::pronunciation::PronunciationService;
use crate::domain::entity::pronunciation::Pronunciation;
use crate::domain::value::PronunciationId;
use crate::infra::repository::pronunciation::PronunciationRepository;

//==============================================================================
// Data
//==============================================================================
pub struct GetPronunciationsReq {
    pub word: String,
    pub speaker_id: Option<PronunciationId>,
}

pub struct GetPronunciationsRes {
    pub pronunciations: Vec<Pronunciation>,
}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct GetPronunciationsProcedure {
    pronunciation_service: Arc<PronunciationService>,
    pronunciation_repository: Arc<PronunciationRepository>,
}

impl Procedure for GetPronunciationsProcedure {
    type Err = core::convert::Infallible;
    type Req = GetPronunciationsReq;
    type Res = GetPronunciationsRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let Self::Req { word, speaker_id } = data;

        self.pronunciation_service.fetch_pronunciations(&word).await;

        let mut pronunciations = self.pronunciation_repository.for_word(word).await.unwrap();

        if let Some(speaker_id) = speaker_id {
            pronunciations = pronunciations
                .into_iter()
                .filter(|it| *it.id() == speaker_id)
                .collect::<Vec<_>>();
        }

        Ok(Self::Res { pronunciations })
    }
}
