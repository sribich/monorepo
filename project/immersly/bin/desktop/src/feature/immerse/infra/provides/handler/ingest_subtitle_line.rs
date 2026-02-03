use axum::{Json, extract::State};
use railgun::typegen::Typegen;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Typegen)]
pub(super) struct Request {
    line: String,
}

#[derive(Typegen, Serialize)]
pub(super) struct Empty {}

pub(super) async fn ingest_subtitle_line(
    State(AppState { .. }): State<AppState>,
    Json(line): Json<Request>,
) -> Json<Empty> {
    println!("{:?}", line);

    // let result = segment_text(line.line);

    // channel.send(result);
    // channel.send(line.line);

    Json(Empty {})
}
