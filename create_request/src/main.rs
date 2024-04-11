#![feature(unwrap_infallible)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use tfhe::library::{RequestPayload, Client};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;


fn main() {

    // create_request <client_key> <opcode> <value-1> <value-2> ... <value-n>
    // create_request client.json 1 3909090 409234909
    let args: Vec<String> = env::args().collect();

    // Deserialize `client.json`

    // Temporary this should be read from the client.json
    //let client = Client::new(123123108858989285289828398u128);

    match args[2].clone().parse::<u32>() {
       Ok(op) => {
          println!("Opcode {}", op);
          let json_file_path = Path::new(args[1].as_str());
           match File::open(json_file_path) {
              Ok(js) => {
                  let reader = BufReader::new(js);
                  match serde_json::from_reader::<BufReader<File>, Client>(reader) {
                      Ok(cli) => {
                          let mut vals = Vec::new();
                          for i in args[3..].iter() {
                             vals.push(i.parse::<u32>().unwrap());
                          }
                          let request_payload = RequestPayload::new(op, cli.key(), vals);
                          let request_payload_file = "request.json";
                          let _ = std::fs::write(
                              request_payload_file,
                              serde_json::to_string_pretty(&request_payload).unwrap(),
                          );
                      }
                      Err(e) => {
                          println!("Error {}", e);
                      }
                  }
              }
               Err(e) => {
                   println!("Error {}", e);
               }
           }
       }
       Err(_) => { println!("Unsupported opcode")}
    }

    //let vals = vec![31337, 31991091];
    //let request_payload = RequestPayload::new(69, client.key(), vals);

    //let request_file = "request.json";
    //println!("Writing request.json");
    //let _ = std::fs::write(
    //    request_file,
    //    serde_json::to_string_pretty(&request_payload).unwrap(),
    //);
}
