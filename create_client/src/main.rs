#![feature(unwrap_infallible)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tfhe_library;

fn main() {
    let client = Client::new(123123108858989285289828398u128);
    println!("Hello, world!");
}
