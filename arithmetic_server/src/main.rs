use axum::extract::DefaultBodyLimit;
use axum::{extract, routing::post, Json, Router};
use tfhe_engine::library::{RequestPayload, ResponsePayload};

#[tokio::main]

async fn main() {
    println!("Starting server");
    let app = Router::new()
        .route("/job", post(process_request))
        .layer(DefaultBodyLimit::max(500000000));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn process_request(
    extract::Json(payload): extract::Json<RequestPayload>,
) -> Json<ResponsePayload> {
    let response = payload.exec();
    Json(response)
}
