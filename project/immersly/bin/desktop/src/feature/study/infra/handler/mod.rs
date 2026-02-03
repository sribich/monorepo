use std::sync::Arc;

use railgun::rpc::{
    procedure::{Procedure, Unresolved},
    router::Router,
};

use crate::infra::provides::http::AppState;

pub mod add_card;
pub mod play_audio;
pub mod score;
pub mod study;

pub fn get_study_router(
    router: Router<AppState>,
    procedure: Procedure<Unresolved>,
    state: Arc<AppState>,
) -> Router<AppState> {
    router
        .procedure("study:AddCard", procedure.mutation(add_card::handler))
        .procedure("study:Score", procedure.mutation(score::handler))
        .procedure("study:Study", procedure.query(study::handler))
        .apply(|router| {
            router.route(
                "/study:PlayAudio/{id}/{kind}",
                axum::routing::get(play_audio::handler).with_state(state),
            )
        })
}
