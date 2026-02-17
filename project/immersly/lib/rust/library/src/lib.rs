#![feature(ptr_metadata, try_as_dyn, macro_metavar_expr_concat)]
use std::sync::Arc;

use app::procedure::add_book::AddBookProcedure;
use app::procedure::list_media::ListMediaProcedure;
use app::procedure::read_book::ReadBookProcedure;
use app::procedure::reprocess_sync::ReprocessSyncProcedure;
use app::procedure::set_progress::SetProgressProcedure;
use axum::extract::Path;
use axum::extract::State;
use axum::http::Uri;
use axum::response::IntoResponse;
use infra::repository::book::BookRepository;
use infra::repository::book_progress::BookProgressRepository;
use prisma_client::model;
use railgun::di::Container;
use railgun::di::InjectorBuilder;
use railgun::di::InjectorError;
use railgun::di::Routes;
use railgun::module;
use railgun::rpc::procedure::Procedure;
use railgun::rpc::procedure::Unresolved;
use railgun::rpc::router::Router;
use shared::domain::value::muid::Muid;
use shared::infra::database::Sqlite;
use shared::infra::http::AppState;
use tower_http::services::ServeFile;

pub mod app;
pub mod domain;
mod infra;

module!(LibraryModule, AppState);

impl Container for LibraryModule {
    fn inject(&self, injector: &mut InjectorBuilder) -> Result<(), InjectorError> {
        injector
            .add::<BookRepository>()?
            .add::<BookProgressRepository>()?
            .add::<AddBookProcedure>()?
            .add::<ListMediaProcedure>()?
            .add::<ReadBookProcedure>()?
            .add::<ReprocessSyncProcedure>()?
            .add::<SetProgressProcedure>()?;

        Ok(())
    }
}

impl Routes<AppState> for LibraryModule {
    fn routes(
        &self,
        router: Router<AppState>,
        procedure: Procedure<Unresolved>,
        state: Arc<AppState>,
    ) -> Router<AppState> {
        router
            .procedure(
                "library:ListMedia",
                procedure.query(infra::handler::list_media::handler),
            )
            .procedure("library:AddBook", procedure.mutation(infra::handler::add_book::add_book_handler))
            .procedure("library:ReadBook", procedure.query(infra::handler::read_book::read_book_handler))
            .procedure(
                "library:ReprocessSync",
                procedure.mutation(infra::handler::reprocess_sync::handler),
            )
            .procedure(
                "library:SetProgress",
                procedure.mutation(infra::handler::set_progress::handler),
            )
            .apply(|router| {
                router
                    .nest_service(
                        "/dictionary_image/{id}/",
                        axum::routing::get(test).with_state(Arc::clone(&state)),
                    )
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
