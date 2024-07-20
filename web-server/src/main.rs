use axum::{extract::Query, routing::get, Router};
use library::add;
use serde::Deserialize;

#[derive(Deserialize)]
struct AddParams {
    a: usize,
    b: usize,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/add",
            get(|params: Query<AddParams>| async move { add(params.a, params.b).to_string() }),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
