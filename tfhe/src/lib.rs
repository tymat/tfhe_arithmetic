#![allow(unused)]


pub mod library {

    use serde::{Deserialize, Serialize};
    use tfhe::{FheUint32, FheUint16, Seed, ConfigBuilder, ClientKey, ServerKey, CompressedServerKey};
    use tfhe::prelude::{FheDecrypt, FheEncrypt, FheTryEncrypt};
    use tfhe::set_server_key;
    use base64::{engine::general_purpose, Engine as _};
    use std::collections::hash_map::Iter;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::iter::Map;
    use std::path::Path;
    use std::time::Instant;
    use serde_json::Result;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Client {
        client_key_b64: String
    }

    impl Client {
        pub fn new(seed: u128) -> Client {
            let builder = ConfigBuilder::default();
            let config = builder.build();
            let client_key = ClientKey::generate_with_seed(config.clone(), Seed(seed));
            let ck_serialized = bincode::serialize(&client_key).unwrap();
            let client_key_b64 = general_purpose::STANDARD.encode(&ck_serialized);

            Client { client_key_b64: client_key_b64 }
        }

        // TODO
        pub fn from_json(json_file: String) -> Client {
            let json_file_path = Path::new(json_file.as_str());
            match File::open(json_file_path) {
                Ok(x) => {
                    let reader = BufReader::new(x);
                    match serde_json::from_reader(reader) {
                        Ok(u) => { u }
                        Err(_) => {
                            println!("Parse Error");
                            Client::new(12345678910)
                        }
                    }
                }
                Err(e) => {
                    println!("Error {}", e);
                    Client::new(12345678910)
                }
            }

        }

        pub fn key(&self) -> ClientKey {
            let ck_serialized = general_purpose::STANDARD.decode(&self.client_key_b64).unwrap();
            let client_key = bincode::deserialize(&ck_serialized[..]).unwrap();
            client_key
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct RequestPayload {
        operation: u32,
        server_key_b64: String,
        args: Vec<String>
    }

    impl RequestPayload {
        pub fn new(operation: u32, client_key: ClientKey, args: Vec<u32>) -> RequestPayload {
            let server_key = CompressedServerKey::new(&client_key);
            let server_key_bytes = bincode::serialize(&server_key.clone()).unwrap();
            let server_key_b64 = general_purpose::STANDARD.encode(&server_key_bytes);
            set_server_key(server_key.decompress());

            let encrypted_args = args.iter().map(|&x|
                {
                    let mut encrypted = FheUint32::encrypt(x, &client_key);
                    let encrypted_bytes = bincode::serialize(&encrypted).unwrap();
                    general_purpose::STANDARD.encode(&encrypted_bytes)
                }
            ).collect();

            RequestPayload {
                operation,
                server_key_b64,
                args: encrypted_args
            }
        }
    }
}