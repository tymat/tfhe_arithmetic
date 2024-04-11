use axum::{
    extract,
    response::{IntoResponse},
//    routing::get,
    routing::post,
    Json, Router,
};
//use tokio::{sync::RwLock, fs::File, io::AsyncReadExt};
use tfhe::library::{RequestPayload, ResponsePayload};

#[tokio::main]

async fn main() {
    println!("Starting server");
    let app = Router::new().route("/job", post(process_request));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn process_request(
    extract::Json(payload): extract::Json<RequestPayload>,
) -> impl IntoResponse {
    println!("{:?}", payload);
    let temp_string = vec!["abc".to_string(), "def".to_string()];
    Json(ResponsePayload {
        operation: 69,
        args: temp_string,
        answer_b64: "hello world".to_string(),
    })
    .into_response();
}
