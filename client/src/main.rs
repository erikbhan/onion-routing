use std::{net::{TcpStream}, io::{Read, Write, stdin}};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

static NODES: [&str; 3] = ["1111", "2222", "3333"];
// static mut KEYS: [&str; 3] = ["key1", "key2", "key3"]

fn main() {
    // get_keys();

    loop {
        println!("Enter your message:");
        let mut message = String::new();
        stdin().read_line(&mut message).expect("Something went wrong when reading message.");

        if message.trim().eq("exit") {
            println!("Exiting program.");
            break;
        }

        println!("Sending: {}", message);
        let msg = message.as_bytes();

        let path = find_path();

        let package = msg; // encrypt(msg, path);

        send(package, path[0])
    }
    println!("Program terminated.");
}

// TODO (Maybe): find path
fn find_path() -> Vec<usize> {
    let mut path = vec![0, 1, 2];

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    irs.shuffle(&mut path, &mut rng).expect("Could not shuffle path");

    path
}

/*
// TODO: Handshake nodes to get 
fn get_keys() {
    for i in 0..2 {
        // let key = sent_to_exchange_key_lib(NODES[i])
        // KEYS[i] = key
    }
}

// TODO: Write encrypting for path
fn encrypt(msg:&[u8], path:[i32; 3]) {
    let mut package = msg;
    for index in path.rev() {
        // set HEADER
            // destination = "localhost:{}", NODES[index]
        // encode package and header
            // private_key = KEYS[index]
            // send_to_encryption_lib(package, private_key)
    }
    return package;
}

// TODO: decrypt
fn decrypt(package:[u8;8], path:[i32; 3]) -> [u8;8] {
    let response;
    for index in path {
        // decode package
            // private_key = KEYS[index]
            // send_to_dencryption_lib(package, private_key)
    }
    return response;
}
*/

// TODO: send to first node in path
fn send(msg:&[u8], node:usize) {
    let addr = format!("localhost:{}", NODES[node]);
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all(msg).unwrap();
            println!("Sent, awaiting reply...");

            // recieve answer:
            let mut data = [0_u8; 8];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let result = data; // decrypt(data);
                    println!("Recieved: {:?}", String::from_utf8(result.to_vec()));
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}