use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;

use futures_util::future::join_all;
use prisma_client::model;
use railgun::di::Component;
use reqwest::Method;
use reqwest::Url;
use shared::infra::Procedure;
use storage::app::procedure::commit_resource::CommitResourceProcedure;
use storage::app::procedure::commit_resource::CommitResourceReq;
use storage::app::procedure::prepare_resource::PrepareResourceProcedure;
use storage::app::procedure::prepare_resource::PrepareResourceReq;
use tokio::io::AsyncWriteExt;

use crate::infra::forvo::ForvoFetcher;
use crate::infra::repository::pronunciation::PronunciationRepository;

#[derive(Component)]
pub struct PronunciationService {
    pronunciation_repository: Arc<PronunciationRepository>,
    commit_resource: Arc<CommitResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
}

fn format_command(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args = cmd
        .get_args()
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");

    format!("{program} {args}")
}

impl PronunciationService {
    pub async fn needs_pronunciations(&self, word: String) -> bool {
        let pronunciations = self
            .pronunciation_repository
            .count_words(word)
            .await
            .unwrap();

        pronunciations == 0
    }

    pub async fn fetch_pronunciations<S: AsRef<str>>(&self, word: S) {
        if !self.needs_pronunciations(word.as_ref().to_owned()).await {
            return ();
        }

        let fetcher = ForvoFetcher {};
        let results = fetcher.get_pronunciations(word.as_ref().to_owned()).await;

        let tasks = results
            .items
            .into_iter()
            .map(|pronunciation| {
                let (commit_resource, prepare_resource, pronunciation_repository) = (
                    self.commit_resource.clone(),
                    self.prepare_resource.clone(),
                    self.pronunciation_repository.clone(),
                );

                tokio::spawn(async move {
                    let resource = prepare_resource
                        .run(PrepareResourceReq {
                            filename: "audio.webm".to_owned(),
                        })
                        .await
                        .unwrap();

                    let data = pronunciation_repository
                        .create(&pronunciation, &resource.resource)
                        .await;

                    let response = reqwest::get(&pronunciation.pathogg)
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap();

                    let mut command = ffmpeg_sidecar::command::FfmpegCommand::new()
                        .arg("-bitexact")
                        .input("pipe:")
                        .codec_audio("libopus")
                        .arg("-b:a")
                        .arg("32k")
                        .no_video()
                        .format("webm")
                        .no_overwrite()
                        .output(resource.path)
                        .spawn()
                        .unwrap();

                    command.take_stdin().unwrap().write_all(&response).unwrap();
                    command.wait().unwrap();

                    let _ = commit_resource
                        .run(CommitResourceReq {
                            resource: resource.resource,
                        })
                        .await
                        .unwrap();
                })
            })
            .collect::<Vec<_>>();

        join_all(tasks).await;
    }
}
