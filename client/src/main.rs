use std::{net::{TcpStream}, io::{Read, Write}, str::from_utf8};

static nodes: &'static [&str] = &["1111", "2222", "3333"];

fn main() {
    
    let is_running = true;

    while is_running {
        println!("Enter your message:");
        let message = std::io::stdin().read_line(&mut line).unwrap();
        let mut msg = [0 as u8; 8];
        msg = message.to_be_bytes();

        if message.to_string() == "exit" {
            break;
        }
        
        // let package = encrypt(msg, find_path())

        send(msg, 0)
    }
    println!("Terminated.");
}

// TODO (Maybe): find path
fn find_path() -> [i32; 3] {
    let path:[i32; 3] = [1, 2, 3];
    return path;
}

// TODO: Write encrypting for path
fn encrypt(mut msg:[u8; 8], path:[i32; 3]) {
    let package;
    for node in nodes {
    }
    return package;
}

// TODO: send to first node in path
fn send(mut msg:[u8; 8], node:i32) {
    let addr = format!("localhost:{}", nodes[0]);
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
                stream.write(msg).unwrap();
                println!("Sent, awaiting reply...");

                // recieve answer:
                let mut data = [0 as u8; 8]; // using 6 byte buffer
                match stream.read_exact(&mut data) {

                    //let result = decrypt(package);

                    Ok(_) => {
                        if &data == msg {
                            println!("Recieved: {}", String::from_utf8(data));
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

// TODO: decrypt
fn decrypt(package:[u8;8]) -> [u8;8] {
    let response;
    for node in nodes {
    }
    return response;
}