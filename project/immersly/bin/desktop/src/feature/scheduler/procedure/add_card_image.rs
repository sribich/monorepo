use std::fmt::write;
use std::sync::Arc;

use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use mime::Mime;
use prisma_client::model;
use railgun_di::Component;

use crate::feature::av::procedure::ExtractAudioProcedure;
use crate::feature::scheduler::repository::scheduler::SchedulerRepository;
use crate::feature::storage::domain::values::ResourceId;
use crate::feature::storage::procedure::commit_resource::CommitResourceProcedure;
use crate::feature::storage::procedure::commit_resource::CommitResourceReq;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceProcedure;
use crate::feature::storage::procedure::prepare_resource::PrepareResourceReq;
use crate::system::Procedure;

//==============================================================================
// Data
//==============================================================================
#[derive(Debug)]
pub struct AddCardImageRequest {
    pub id: Muid,
    pub image: Vec<u8>,
    pub mime_type: String,
}

pub struct AddCardImageResponse {}

//==============================================================================
// Procedure
//==============================================================================
#[derive(Component)]
pub struct AddCardImageProcedure {
    db: Arc<Sqlite>,
    commit_resource: Arc<CommitResourceProcedure>,
    prepare_resource: Arc<PrepareResourceProcedure>,
}

impl Procedure for AddCardImageProcedure {
    type Err = core::convert::Infallible;
    type Req = AddCardImageRequest;
    type Res = AddCardImageResponse;

    async fn run(&self, data: Self::Req) -> core::result::Result<Self::Res, Self::Err> {
        let mime_type = data.mime_type.parse().unwrap();
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

        self.db
            .client()
            .card()
            .update(
                model::card::id::equals(data.id.as_bytes().to_owned()),
                vec![model::card::image_id::set(Some(
                    resource.resource.as_bytes().to_owned(),
                ))],
            )
            .exec()
            .await
            .unwrap();

        Ok(Self::Res {})
    }
}

fn get_extension(mime: &Mime) -> &'static str {
    match (mime.type_(), mime.subtype()) {
        (mime::IMAGE, mime::JPEG) => "jpg",
        (_, _) => panic!("Unknown mime type {mime}"),
    }
}
