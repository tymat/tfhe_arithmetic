#![feature(unwrap_infallible)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tfhe::library::Client;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].clone().parse::<u128>() {
       Ok(x) => {
           let client =  Client::new(x.clone());
           let client_file = "client.json";
           println!("Writing {}", client_file);
           let _ = std::fs::write(
               client_file,
               serde_json::to_string_pretty(&client).unwrap(),
           );
       }
       Err(e) => {
           println!("Error in seed number: {}", e);
       }
    }
}
