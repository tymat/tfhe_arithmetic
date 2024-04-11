#![feature(unwrap_infallible)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tfhe::library::Client;

fn main() {
    let client =  Client::new(123123108858989285289828398u128);
    println!("Generating a new seed");

    let client_file = "client.json";

    println!("Writing client.json");

    let _ = std::fs::write(
        client_file,
        serde_json::to_string_pretty(&client).unwrap(),
    );
}
