use std::env;
use std::fs::File;
use std::io::BufReader;
use tfhe::ClientKey;
use tfhe_engine::library::{Client, ResponsePayload};

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_file_path = args[1].clone();
    let client_key_path = args[2].clone();

    match File::open(json_file_path) {
        Ok(x) => {
            let reader = BufReader::new(x);
            match serde_json::from_reader::<BufReader<File>, ResponsePayload>(reader) {
                Ok(y) => match File::open(client_key_path) {
                    Ok(z) => {
                        let reader = BufReader::new(z);
                        match serde_json::from_reader::<BufReader<File>, Client>(reader) {
                            Ok(c) => {
                                let client_key: ClientKey = c.key();
                                y.reveal_answer(client_key);
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}
