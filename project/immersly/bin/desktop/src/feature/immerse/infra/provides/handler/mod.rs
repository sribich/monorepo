pub mod ingest_subtitle_line;

use ingest_subtitle_line::ingest_subtitle_line;
use railgun::rpc::{
    procedure::{Procedure, Unresolved},
    router::Router,
};

pub fn get_immerse_router(
    router: Router<AppState>,
    procedure: Procedure<Unresolved>,
) -> Router<AppState> {
    router.procedure(
        "immerse:IngestSubtitleLine",
        procedure.mutation(ingest_subtitle_line),
    )
}
