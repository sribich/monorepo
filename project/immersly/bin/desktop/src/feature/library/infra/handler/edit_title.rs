use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use railgun::typegen::Typegen;
use railgun_api::ApiError;
use railgun_api::json::ApiErrorKind;
use railgun_api::json::ApiResponse;
use railgun_api::json::ApiResult;
use railgun_di::Component;
use serde::Deserialize;
use serde::Serialize;

//==============================================================================
// Handler Request
//==============================================================================
#[derive(Debug, Deserialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct EditTitleRequest {
    title_id: String,
}

//==============================================================================
// Handler Response
//==============================================================================
#[derive(Debug, Serialize, Typegen)]
#[serde(rename_all = "camelCase")]
pub struct EditTitleResponse {}

//==============================================================================
// Handler -> Domain -> Handler
//==============================================================================
impl TryInto<()> for EditTitleRequest {
    type Error = core::convert::Infallible;

    fn try_into(self) -> Result<(), Self::Error> {
        todo!();
    }
}

impl TryFrom<()> for EditTitleResponse {
    type Error = core::convert::Infallible;

    fn try_from(value: ()) -> Result<Self, Self::Error> {
        todo!();
    }
}

//==============================================================================
// Error Handling
//==============================================================================
#[derive(ApiError, Serialize, Typegen)]
pub enum ApiError {}

impl From<core::convert::Infallible> for ApiError {
    fn from(value: core::convert::Infallible) -> Self {
        unreachable!();
    }
}

//==============================================================================
// Axum State
//==============================================================================
#[derive(Clone, Component)]
#[component(from_state)]
pub struct EditTitleState {
    // commit_resource: Arc<CommitResourceProcedure>,
    // prepare_resource: Arc<PrepareResourceProcedure>,
    // resource_repository: Arc<ResourceRepository>,
    // library_repository: Arc<LibraryRepository>,
}

// let data = self.repository.reader().get_title(&data).await.unwrap();

//==============================================================================
// Handler
//==============================================================================
pub async fn handler(
    State(_state): State<EditTitleState>,
    Path(_path): Path<String>,
    mut _multipart: Multipart,
) -> ApiResult<EditTitleResponse, ApiError> {
    todo!();

    /*
    // let mut file_name = String::new();
    // let mut chunk_number = 0;
    // let mut total_chunks = 0;
    // let mut chunk_data = Vec::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(err) => {
            println!("{:#?}", err);
            // eprintln!("Error reading multipart field: {:?}", err);
            // return StatusCode::BAD_REQUEST;
            unreachable!();
        },
    } {
        let field_name = field.name().unwrap_or_default().to_string();

        match field_name.as_str() {
            "file" => {
                let file_name = field.file_name().unwrap();

                let resource = state
                    .prepare_resource
                    .run(<PrepareResourceProcedure as Procedure>::Req {
                        filename: file_name.to_owned(),
                    })
                    .await
                    .unwrap();

                let mut file = tokio::fs::File::create(&resource.path).await.unwrap();
                let mut stream = field.into_stream();

                while let Some(chunk) = stream.next().await {
                    let data = chunk.unwrap();
                    file.write_all(&data).await.unwrap();
                }

                state
                    .commit_resource
                    .run(<CommitResourceProcedure as Procedure>::Req {
                        resource: resource.resource.clone(),
                    })
                    .await
                    .unwrap();

                state
                    .library_repository
                    .writer()
                    .update_title(&LibraryId::from_str(&path).unwrap(), &resource.resource)
                    .await;

                println!("{:#?}", resource.resource);
            },
            _ => unreachable!(),
        }
    }
    */

    /*
    if file_name.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    let temp_dir = format!("./uploads/temp/{}", file_name);
    fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {});
    let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
    let mut file = File::create(&chunk_path).unwrap();
    file.write_all(&chunk_data).unwrap();

    if is_upload_complete(&temp_dir, total_chunks) {
        assemble_file(&temp_dir, &file_name, total_chunks).unwrap();
    }
     */

    ApiResponse::success(StatusCode::OK, EditTitleResponse {})
}

/*
fn is_upload_complete(temp_dir: &str, total_chunks: usize) -> bool {
    match fs::read_dir(temp_dir) {
        Ok(entries) => entries.count() == total_chunks,
        Err(_) => false,
    }
}

fn assemble_file(temp_dir: &str, file_name: &str, total_chunks: usize) -> std::io::Result<()> {
    let output_path = format!("./uploads/{}", file_name);
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&output_path)?;

    for chunk_number in 0..total_chunks {
        let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
        let chunk_data = fs::read(&chunk_path)?;
        output_file.write_all(&chunk_data)?;
    }

    fs::remove_dir_all(temp_dir)?;
    Ok(())
}
*/

/*

    }
}

// ffmpeg_sidecar::command::FfmpegCommand::new()
// .seek(format!("{}ms", data.reading_timestamp.0))
// .input(...)
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

*/
