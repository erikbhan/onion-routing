use std::{net::{TcpStream}, io::{Read, Write, stdin}, str::from_utf8};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

static NODES: &'static [&str] = &["1111", "2222", "3333"];

fn main() {
    loop {
        println!("Enter your message:");
        let mut message = String::new();
        stdin().read_line(&mut message).expect("Something went wrong when reading message.");

        if message.trim().eq("exit") {
            println!("Exiting program.");
            break;
        }

        println!("Sending: {}", message);
        let mut msg = message.as_bytes();

        let path = find_path();

        // let package = encrypt(msg, path)

        send(msg, path[0])
    }
    println!("Program terminated.");
}

// TODO (Maybe): find path
fn find_path() -> Vec<usize> {
    let mut path = vec![0, 1, 2];

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();
    irs.shuffle(&mut path, &mut rng).expect("Could not shuffle path");

    return path;
}

/*
// TODO: Write encrypting for path
fn encrypt(mut msg:&[u8], path:[i32; 3]) {
    let package;
    for node in nodes {
    }
    return package;
}

// TODO: decrypt
fn decrypt(package:[u8;8]) -> [u8;8] {
    let response;
    for node in nodes {
    }
    return response;
}
*/

// TODO: send to first node in path
fn send(mut msg:&[u8], node:usize) {
    let addr = format!("localhost:{}", NODES[node]);
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
                stream.write(&msg).unwrap();
                println!("Sent, awaiting reply...");

                // recieve answer:
                let mut data = [0 as u8; 8]; // using 6 byte buffer
                match stream.read_exact(&mut data) {

                    //let result = decrypt(package);

                    Ok(_) => {
                        if &data == &msg {
                            println!("Recieved: {:?}", String::from_utf8(data.to_vec()));
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        ,
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}