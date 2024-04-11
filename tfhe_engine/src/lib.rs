#![allow(unused)]

pub mod library {

    use base64::{engine::general_purpose, Engine as _};
    use serde::{Deserialize, Serialize};
    use serde_json::Result;
    use std::collections::hash_map::Iter;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::iter::Map;
    use std::path::Path;
    use std::time::Instant;
    use tfhe::prelude::{FheDecrypt, FheEncrypt, FheTryEncrypt};
    use tfhe::set_server_key;
    use tfhe::{
        ClientKey, CompressedServerKey, ConfigBuilder, FheUint16, FheUint32, Seed, ServerKey,
    };

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Client {
        client_key_b64: String,
    }

    impl Client {
        pub fn new(seed: u128) -> Client {
            let builder = ConfigBuilder::default();
            let config = builder.build();
            let client_key = ClientKey::generate_with_seed(config.clone(), Seed(seed));
            let ck_serialized = bincode::serialize(&client_key).unwrap();
            let client_key_b64 = general_purpose::STANDARD.encode(&ck_serialized);

            Client { client_key_b64 }
        }

        pub fn key(&self) -> ClientKey {
            let ck_serialized = general_purpose::STANDARD
                .decode(&self.client_key_b64)
                .unwrap();
            let client_key = bincode::deserialize(&ck_serialized[..]).unwrap();
            client_key
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct RequestPayload {
        pub operation: u32,
        pub server_key_b64: String,
        pub args: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ResponsePayload {
        pub operation: u32,
        pub args: Vec<String>,
        pub answer_b64: String,
    }

    impl ResponsePayload {
        pub fn load_json_file(path: String) -> ResponsePayload {
            unimplemented!()
        }
        pub fn reveal_answer(&self, client_key: ClientKey) {
            let answer_bin = general_purpose::STANDARD.decode(&self.answer_b64).unwrap();

            let fheu32_answer: FheUint32 = bincode::deserialize(&answer_bin[..]).unwrap();
            let answer: u32 = fheu32_answer.decrypt(&client_key);

            println!("answer = {}", answer)
        }
    }

    impl RequestPayload {
        pub fn new(operation: u32, client_key: ClientKey, args: Vec<u32>) -> RequestPayload {
            let server_key = CompressedServerKey::new(&client_key);
            let server_key_bytes = bincode::serialize(&server_key.clone()).unwrap();
            let server_key_b64 = general_purpose::STANDARD.encode(&server_key_bytes);
            set_server_key(server_key.decompress());

            let encrypted_args = args
                .iter()
                .map(|&x| {
                    let mut encrypted = FheUint32::encrypt(x, &client_key);
                    let encrypted_bytes = bincode::serialize(&encrypted).unwrap();
                    general_purpose::STANDARD.encode(&encrypted_bytes)
                })
                .collect();

            RequestPayload {
                operation,
                server_key_b64,
                args: encrypted_args,
            }
        }

        pub fn exec(&self) -> ResponsePayload {
            match self.operation {
                1 => {
                    let builder = ConfigBuilder::default();

                    let config = builder.build();

                    let server_key_bin = general_purpose::STANDARD
                        .decode(&self.server_key_b64)
                        .unwrap();

                    let compressed_server_key: CompressedServerKey =
                        bincode::deserialize(&server_key_bin[..]).unwrap();

                    let server_key = compressed_server_key.decompress();
                    set_server_key(server_key.clone());

                    let mut fheu32_args: Vec<FheUint32> = Vec::new();

                    for i in self.args.iter() {
                        let item = general_purpose::STANDARD.decode(&i).unwrap();
                        let fheu32_item: FheUint32 = bincode::deserialize(&*item).unwrap();
                        fheu32_args.push(fheu32_item);
                    }

                    let answer = fheu32_args[0].clone() + fheu32_args[1].clone();

                    let answer_bytes = bincode::serialize(&answer.clone()).unwrap();
                    let answer_b64 = general_purpose::STANDARD.encode(&answer_bytes);

                    ResponsePayload {
                        operation: self.operation,
                        answer_b64,
                        args: self.clone().args,
                    }
                }
                _ => {
                    println!("Unsupported opcode");
                    ResponsePayload {
                        operation: 32,
                        answer_b64: "wrong".to_string(),
                        args: vec!["abc".to_string(), "def".to_string()],
                    }
                }
            }
        }
    }
}
