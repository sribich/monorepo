use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;
use storage::app::procedure::extract_audio::ExtractAudioProcedure;
use storage::app::procedure::extract_audio::ExtractAudioReq;
use storage::domain::value::ResourceId;

use crate::infra::repository::scheduler::SchedulerRepository;

pub struct CreateCardReq {
    pub word: String,
    pub reading: String,
    pub reading_audio: Option<ResourceId>,
    pub sentence: String,
    pub sentence_audio: Option<AudioSegment>,
}

pub struct AudioSegment {
    pub resource: ResourceId,
    pub start_time: usize,
    pub end_time: usize,
}

#[derive(Component)]
pub struct CreateCardProcedure {
    extract_audio: Arc<ExtractAudioProcedure>,
    scheduler_repository: Arc<SchedulerRepository>,
}

impl Procedure for CreateCardProcedure {
    type Err = core::convert::Infallible;
    type Req = CreateCardReq;
    type Res = ();

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let audio = data.sentence_audio.unwrap();

        let audio_id = self
            .extract_audio
            .run(ExtractAudioReq {
                source: audio.resource.clone(),
                timestamp_start: audio.start_time,
                timestamp_end: audio.end_time,
            })
            .await
            .unwrap()
            .resource_id;

        self.scheduler_repository
            .create_card(
                data.word,
                data.reading,
                data.sentence,
                data.reading_audio.unwrap(),
                audio_id.into(),
            )
            .await;

        Ok(())
    }
}
