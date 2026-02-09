pub mod application;
pub mod domain;
mod infra;
pub mod use_case;

use std::sync::Arc;

use application::procedure::add_book::AddBookProcedure;
use application::procedure::list_media::ListMedia;
use application::procedure::read_book::ReadBookProcedure;
use axum::extract::DefaultBodyLimit;
use axum::extract::Path;
use axum::extract::State;
use axum::http::Uri;
use axum::response::IntoResponse;
use features::shared::domain::value::muid::Muid;
use features::shared::infra::database::Sqlite;
use features::shared::infra::http::AppState;
use infra::handler::add_book::add_book_handler;
use infra::handler::edit_title;
use infra::handler::play_audio::play_audio_handler;
use infra::handler::read_book::read_book_handler;
use infra::handler::reprocess_sync::reprocess_sync_handler;
use infra::handler::{self};
use infra::repository::book::SqliteBookReader;
use infra::repository::book::SqliteBookRepository;
use infra::repository::book::SqliteBookWriter;
use prisma_client::model;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use railgun_di::InjectorBuilder;
use railgun_di::InjectorError;
use tower_http::services::ServeFile;
use use_case::reprocess_sync::ReprocessSyncUseCase;
use use_case::set_progress::SetProgressUseCase;

use crate::startup::Feature;

pub struct LibraryFeature {}

impl LibraryFeature {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Feature for LibraryFeature {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<SqliteBookRepository>()?
            .add::<SqliteBookReader>()?
            .add::<SqliteBookWriter>()?
            .add::<AddBookProcedure>()?
            .add::<ListMedia>()?
            .add::<ReadBookProcedure>()?
            .add::<ReprocessSyncUseCase>()?
            .add::<SetProgressUseCase>()?;

        Ok(())
    }


    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .procedure(
                "library:ListMedia",
                procedure.query(handler::list_media::handler),
            )
            .procedure("library:AddBook", procedure.mutation(add_book_handler))
            .procedure("library:ReadBook", procedure.query(read_book_handler))
            .procedure(
                "library:ReprocessSync",
                procedure.mutation(reprocess_sync_handler),
            )
            .procedure(
                "library:SetProgress",
                procedure.mutation(handler::set_progress::handler),
            )
            .apply(|router| {
                // .procedure("library:EditTitle", procedure.mutation(edit_title::handler))
                router
                    .nest_service(
                        "/dictionary_image/{id}/",
                        axum::routing::get(test).with_state(Arc::clone(&state)),
                    )
                    .nest_service(
                        "/edit_title/{id}",
                        axum::routing::post(edit_title::handler)
                            .with_state(Arc::clone(&state))
                            .layer(DefaultBodyLimit::max(1024 * 1024 * 1024 * 1024)),
                    )
                    //
                    .route("/play/{id}", axum::routing::get(play_audio_handler))
            })
    }
}

pub async fn test(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    uri: Uri,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let data_path = state
        .injector
        .get::<Sqlite>()
        .unwrap()
        .client()
        .dictionary()
        .find_unique(model::dictionary::id::equals(
            Muid::try_from_str(&id).unwrap().as_bytes().to_vec(),
        ))
        .exec()
        .await
        .unwrap()
        .unwrap()
        .data_path;

    let path = urlencoding::decode(request.uri().path())
        .unwrap()
        .to_string();

    ServeFile::new(format!("{data_path}/{path}"))
        .try_call(request)
        .await
        .unwrap()
}
