use crate::statics::file::StaticFile;
use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

#[derive(TypedPath, Deserialize)]
#[typed_path("/static/{*path}")]
pub struct StaticFilePath {
    pub path: String,
}

pub async fn static_path(StaticFilePath { path }: StaticFilePath) -> impl IntoResponse {
    let data = StaticFile::get(&format!("/static/{}", path));

    if let Some(data) = data {
        let file = match tokio::fs::File::open(data.file_name).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap();
            }
        };

        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(Body::from_stream(ReaderStream::new(file)))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
