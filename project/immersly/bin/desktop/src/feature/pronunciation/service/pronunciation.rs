use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;

use features::shared::infra::database::Sqlite;
use futures_util::future::join_all;
use itertools::Itertools;
use prisma_client::model;
use railgun_di::Component;
use reqwest::Method;
use reqwest::Url;
use serde_json::Value;
use tokio::io::AsyncWriteExt;

use crate::feature::pronunciation::infra::forvo::ForvoFetcher;
use crate::feature::pronunciation::repository::pronunciation::PronunciationRepository;
use crate::feature::settings::app::service::settings::SettingService;
use crate::feature::storage::procedure::commit_resource::CommitResourceProcedure;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceProcedure;
use crate::system::Procedure;

#[derive(Component)]
pub struct PronunciationService {
    db: Arc<Sqlite>,
    repository: Arc<PronunciationRepository>,
    settings: Arc<SettingService>,

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
        let num_pronunciations = self
            .db
            .client()
            .pronunciation()
            .count(vec![
                model::pronunciation::word::equals(word),
                // model::pronunciation::reading::equals(Some(reading)),
            ])
            .exec()
            .await
            .unwrap();

        num_pronunciations == 0
    }

    pub async fn fetch_pronunciations<S: AsRef<str>>(&self, word: S) {
        let fetcher = ForvoFetcher {};
        let results = fetcher.get_pronunciations(word.as_ref().to_owned()).await;

        let tasks = results
            .items
            .into_iter()
            .map(|pronunciation| {
                // DISABLED FOR NOW. API IS FUCKED.
                let (commit_resource, prepare_resource, repository) = (
                    self.commit_resource.clone(),
                    self.prepare_resource.clone(),
                    self.repository.clone(),
                );

                tokio::spawn(async move {
                    // Forvo API broke
                    // return;

                    let resource = prepare_resource
                        .run(<PrepareResourceProcedure as Procedure>::Req {
                            filename: "audio.webm".to_owned(),
                        })
                        .await
                        .unwrap();
                    let data = repository
                        .writer()
                        .create(&pronunciation, resource.resource.to_vec())
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

                    commit_resource
                        .run(<CommitResourceProcedure as Procedure>::Req {
                            resource: resource.resource,
                        })
                        .await
                        .unwrap();
                })
            })
            .collect::<Vec<_>>();

        join_all(tasks).await;
    }

    pub async fn list_pronunciations(&self, word: String) -> Vec<model::pronunciation::Data> {
        self.db
            .client()
            .pronunciation()
            .find_many(vec![model::pronunciation::word::equals(word)])
            .exec()
            .await
            .unwrap()
    }
}
