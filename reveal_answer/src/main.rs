use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tfhe_engine::library::{ResponsePayload, Client };
use tfhe::set_server_key;
use tfhe::{
    ClientKey, CompressedServerKey, ConfigBuilder, FheUint16, FheUint32, Seed, ServerKey,
};

fn main() {

    let args: Vec<String> = env::args().collect();
    let json_file_path = args[1].clone();
    let client_key_path = args[2].clone();

    match File::open(json_file_path) {
        Ok(x) => {
            let reader = BufReader::new(x);
            match serde_json::from_reader::<BufReader<File>, ResponsePayload>(reader) {
                Ok(y) => {
                    match File::open(client_key_path) {
                        Ok(z) => {
                            let reader = BufReader::new(z);
                            match serde_json::from_reader::<BufReader<File>, Client>(reader) {
                                Ok(c) => {
                                    println!("{:?}", c);
                                    let client_key: ClientKey = c.key();
                                    y.reveal_answer(client_key);
                                }
                                Err(_) => {

                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

  /*
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
        Err(_) => {
            println!("Unsupported opcode")
        }
    }
}
   */
    println!("Hello, world!");
}
