use std::sync::Arc;

use railgun_di::Component;

use features::storage::domain::value::ResourceId;
use crate::feature::storage::procedure::commit_resource::CommitResourceProcedure;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceProcedure;
use crate::feature::storage::repository::resource::ResourceRepository;
use crate::system::Procedure;

//==============================================================================
// Data
//==============================================================================
pub struct Req {
    pub source: ResourceId,
    pub timestamp_start: usize,
    pub timestamp_end: usize,
}

pub struct Res {
    pub resource_id: ResourceId,
}

//==============================================================================
// UseCase
//==============================================================================
#[derive(Component)]
pub struct ExtractAudioProcedure {
    commit_resource: Arc<CommitResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
    resource_repository: Arc<ResourceRepository>,
}

impl Procedure for ExtractAudioProcedure {
    type Err = core::convert::Infallible;
    type Req = Req;
    type Res = Res;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let base_resource = self
            .resource_repository
            .reader()
            .from_id(&data.source)
            .await
            .unwrap()
            .unwrap();

        let resource = self
            .prepare_resource
            .run(<PrepareResourceProcedure as Procedure>::Req {
                filename: "audio.webm".to_owned(),
            })
            .await
            .unwrap();

        ffmpeg_sidecar::command::FfmpegCommand::new()
            .seek(format!("{}ms", data.timestamp_start))
            .input(base_resource.path)
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
            .run(<CommitResourceProcedure as Procedure>::Req {
                resource: resource.resource.clone(),
            })
            .await
            .unwrap();

        Ok(Res {
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
