use std::fmt::write;
use std::sync::Arc;

use mime::Mime;
use railgun::di::Component;
use shared::infra::Procedure;
use shared::infra::database::Sqlite;
use storage::app::procedure::commit_resource::CommitResourceProcedure;
use storage::app::procedure::commit_resource::CommitResourceReq;
use storage::app::procedure::prepare_resource::PrepareResourceProcedure;
use storage::app::procedure::prepare_resource::PrepareResourceReq;

use crate::domain::value::card_id::CardId;
use crate::infra::repository::scheduler::SchedulerRepository;

#[derive(Debug)]
pub struct AddCardImageReq {
    pub id: CardId,
    pub image: Vec<u8>,
    pub mime_type: Mime,
}

#[derive(Component)]
pub struct AddCardImageProcedure {
    scheduler_repository: Arc<SchedulerRepository>,
    commit_resource: Arc<CommitResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
}

impl Procedure for AddCardImageProcedure {
    type Err = core::convert::Infallible;
    type Req = AddCardImageReq;
    type Res = ();

    async fn run(&self, data: Self::Req) -> Result<Self::Res, Self::Err> {
        let mime_type = data.mime_type;
        let extension = get_extension(&mime_type);

        let resource = self
            .prepare_resource
            .run(PrepareResourceReq {
                filename: format!("resource.{extension}"),
            })
            .await
            .unwrap();

        std::fs::write(resource.path, data.image).unwrap();

        self.commit_resource
            .run(CommitResourceReq {
                resource: resource.resource.clone(),
            })
            .await
            .unwrap();

        self.scheduler_repository
            .set_image(&data.id, &resource.resource)
            .await
            .unwrap();

        Ok(())
    }
}

fn get_extension(mime: &Mime) -> &'static str {
    match (mime.type_(), mime.subtype()) {
        (mime::IMAGE, mime::JPEG) => "jpg",
        (_, _) => panic!("Unknown mime type {mime}"),
    }
}
