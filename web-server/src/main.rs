use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use library::{format_to_mimetype, ImageId, Library};
use memory_adaptor::MemoryAdaptor;
use tokio::net::TcpListener;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};

type Lib = Arc<Mutex<MemoryAdaptor>>;

#[tokio::main]
async fn main() {
    // let library = get_library();

    let media_routes = Router::new()
        .route("/:image_id", get(get_image).delete(remove_image))
        .route("/", post(upload_image));

    let tag_routes = Router::new();

    let app = Router::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .nest("/api/media", media_routes)
        .nest("/api/tags", tag_routes)
        .nest("/file", create_file_router())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(Mutex::new(MemoryAdaptor::default())));

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
#[debug_handler]
async fn upload_image(
    State(library): State<Lib>,
    mut file_data: Multipart,
) -> Json<library::ImageMetadata> {
    let image = file_data.next_field().await.unwrap().unwrap();
    let ext = image.file_name().unwrap().rsplit_once('.').unwrap().1;
    let image_data = library::ImageData {
        metadata: library::ImageMetadata {
            id: 0,
            available_formats: vec![ext.to_string()],
            tags: vec![],
        },
        data: image.bytes().await.unwrap().into(),
    };
    let res;
    {
        let mut library = library.lock().await;
        res = library.add_image(image_data).await.unwrap();
    }
    Json(res)
}

// fn get_library() -> impl Library {
//     todo!()
// }

#[debug_handler]
async fn get_image_data(
    Path(image): Path<String>,
    State(library): State<Lib>,
) -> impl IntoResponse {
    let (id, format) = image.rsplit_once('.').unwrap();
    let id = id.parse::<ImageId>().unwrap();

    let image;
    {
        let library = library.lock().await;
        image = library
            .get_image_data(id, format.to_string())
            .await
            .unwrap();
    }

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, format_to_mimetype(format))],
        image,
    )
}

fn create_file_router() -> Router<Lib> {
    Router::new().route("/:image", get(get_image_data))
}
