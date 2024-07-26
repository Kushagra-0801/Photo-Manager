#![allow(unused)]

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use library::{format_to_mimetype, ImageId, Library};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // let library = get_library();

    let media_routes = Router::new()
        .route("/:image_id", get(get_image).delete(remove_image))
        .route("/", post(upload_image));

    let tag_routes = Router::new();

    let app = Router::new()
        .nest("/api/media", media_routes)
        .nest("/api/tags", tag_routes)
        .nest("/file", create_file_router())
        .with_state(());

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Returns the metadata containing { id, thumbnail?, [{ format, size, dimensions, url }] }
async fn get_image(Path(image_id): Path<String>) {
    let _ = image_id;
    todo!()
}

/// Removes from the library
async fn remove_image(Path(image_id): Path<String>) {
    let _ = image_id;
    todo!()
}

/// Adds to the library
async fn upload_image(file_data: Multipart) {
    let _ = file_data;
    todo!()
}

// fn get_library() -> impl Library {
//     todo!()
// }

#[debug_handler]
async fn get_image_data(Path(image): Path<String>, State(library): State<()>) -> impl IntoResponse {
    let (id, format) = image.rsplit_once('.').unwrap();
    let id = id.parse::<ImageId>().unwrap();
    let image = library
        .get_image_data(id, format.to_string())
        .await
        .unwrap();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, format_to_mimetype(format))],
        image,
    )
}

fn create_file_router() -> Router {
    Router::new().route("/:image", get(get_image_data))
}
