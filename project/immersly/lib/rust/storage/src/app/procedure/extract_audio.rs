use std::sync::Arc;

use railgun::di::Component;
use shared::infra::Procedure;

use super::commit_resource::CommitResourceProcedure;
use super::get_resource::GetResourceProcedure;
use super::prepare_resource::PrepareResourceProcedure;
use crate::app::procedure::commit_resource::CommitResourceReq;
use crate::app::procedure::get_resource::GetResourceReq;
use crate::app::procedure::prepare_resource::PrepareResourceReq;
use crate::domain::value::ResourceId;

pub struct ExtractAudioReq {
    pub source: ResourceId,
    pub timestamp_start: usize,
    pub timestamp_end: usize,
}

pub struct ExtractAudioRes {
    pub resource_id: ResourceId,
}

#[derive(Component)]
pub struct ExtractAudioProcedure {
    commit_resource: Arc<CommitResourceProcedure>,
    get_resource: Arc<GetResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
}

impl Procedure for ExtractAudioProcedure {
    type Err = core::convert::Infallible;
    type Req = ExtractAudioReq;
    type Res = ExtractAudioRes;

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let base_resource = self
            .get_resource
            .run(GetResourceReq {
                id: data.source.clone(),
            })
            .await
            .unwrap()
            .unwrap();

        let resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: "audio.webm".to_owned(),
            })
            .await
            .unwrap();

        ffmpeg_sidecar::command::FfmpegCommand::new()
            .seek(format!("{}ms", data.timestamp_start))
            .input(base_resource.path().to_str().unwrap())
            .duration(format!("{}ms", data.timestamp_end - data.timestamp_start))
            .output(&resource.path)
            .codec_audio("libopus")
            .arg("-b:a")
            .arg("32k")
            .no_video()
            .format("webm")
            .no_overwrite()
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        self.commit_resource
            .run(CommitResourceReq {
                resource: resource.resource.clone(),
            })
            .await
            .unwrap();

        Ok(ExtractAudioRes {
            resource_id: resource.resource,
        })
    }
}

// ffmpeg_sidecar::command::FfmpegCommand::new()
// .seek(format!("{}ms", data.reading_timestamp.0))
// .input("...")
// .duration(format!(
// "{}ms",
// data.reading_timestamp.1 - data.reading_timestamp.0
// ))
// .output(word_audio.to_str().unwrap())
// .spawn()
// .unwrap()
// .iter()
// .unwrap()
// .for_each(|event| {
// println!("{:#?}", event);
// });

// ffmpeg -threads 0 -bitexact -i input.m4a -c:a libopus -b:a 96k -vn -f webm output5.webm
// ffmpeg -progress -superfast -threads 16 -bitexact -i input.m4a -c:a libopus -b:a 32k -vn -f webm output5.webm

// BAD   ffmpeg -bitexact -i audio.ogg -c:a libopus -b:a 32k -f webm -
// GOOD  ffmpeg -bitexact -i audio.ogg -c:a libopus -b:a 32k -vn -f webm -
