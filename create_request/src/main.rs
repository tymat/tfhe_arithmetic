#![feature(unwrap_infallible)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tfhe::library::{RequestPayload, Client};

fn main() {
    let client = Client::new(123123108858989285289828398u128);

    let args = vec![31337, 31991091];
    let request_payload = RequestPayload::new(69, client.key(), args);

    let request_file = "request.json";
    println!("Writing request.json");
    let _ = std::fs::write(
        request_file,
        serde_json::to_string_pretty(&request_payload).unwrap(),
    );
}
