use axum::{
    extract,
    //    routing::get,
    routing::post,
    Json,
    Router,
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
) -> Json<ResponsePayload> {
    println!("{:?}", payload);
    let response = payload.exec();
    println!("{:?}", response.clone());
    Json(response)
}
