use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use railgun_di::Component;

use crate::feature::av::procedure::ExtractAudioProcedure;
use crate::feature::scheduler::repository::scheduler::SchedulerRepository;
use features::storage::domain::value::ResourceId;
use crate::system::Procedure;

//==============================================================================
// Data
//==============================================================================
#[derive(Debug)]
pub struct CreateCardRequest {
    pub word: String,
    pub reading: String,
    pub reading_audio: Option<Muid>,
    pub sentence: String,
    pub sentence_audio: Option<AudioSegment>,
}

#[derive(Debug)]
pub struct AudioSegment {
    pub resource: ResourceId,
    pub start_time: usize,
    pub end_time: usize,
}

pub struct CreateCardResponse {}

//==============================================================================
// Procedure
//==============================================================================
#[derive(Component)]
pub struct CreateCardProcedure {
    extract_audio: Arc<ExtractAudioProcedure>,
    scheduler_repository: Arc<SchedulerRepository>,
}

impl Procedure for CreateCardProcedure {
    type Err = core::convert::Infallible;
    type Req = CreateCardRequest;
    type Res = CreateCardResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let audio = data.sentence_audio.unwrap();

        let audio_id = self
            .extract_audio
            .run(<ExtractAudioProcedure as Procedure>::Req {
                source: audio.resource.clone(),
                timestamp_start: audio.start_time,
                timestamp_end: audio.end_time,
            })
            .await
            .unwrap()
            .resource_id;
        // reading_audio in this case is the pronunciation id

        self.scheduler_repository
            .writer()
            .create_card(
                data.word,
                data.reading,
                data.sentence,
                data.reading_audio.unwrap(),
                audio_id.into(),
            )
            .await;

        Ok(Self::Res {})
    }
}
