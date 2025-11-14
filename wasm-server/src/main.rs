use axum::{
    extract::Path,
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
    Json,
    Router
};
use tokio_util::io::ReaderStream;
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs::File;
#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse { message: "Hello from wasm server".to_string() })
}

async fn stream_wasm_handler(Path(file): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from("./static").join(file);

    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = axum::body::Body::from_stream(stream);

            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/wasm"),
                    (header::CACHE_CONTROL, "no_cache"),
                ],
                body,
            ).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Not Found",
        ).into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wasm/{file}", get(stream_wasm_handler))
        .route("/graph.json", get(|| async {
            axum::Json(serde_json::json!({
                "start": "node!",
                "nodes": {
                    "node1": { "wasm": "/wasm/module1.wasm", "next": "node2" },
                    "node2": { "wasm": "/wasm/module2.wasm", "next": "node3" },
                    "node3": { "wasm": "/wasm/module3.wasm", "next": null }
                }
            }))
        }))
        .route("/hello", get(hello_handler));

    let url = "0.0.0.0:3002";
    let listener = tokio::net::TcpListener::bind(url)
        .await
        .expect("Failed to bind port");
    println!("Axum server running at http://{}", url);

    axum::serve(listener, app)
        .await
        .expect("Server error");

}
