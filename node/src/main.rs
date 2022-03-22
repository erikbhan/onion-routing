use std::{io::stdin, net::{TcpListener, TcpStream}, thread};
use local_ip_address::local_ip;
use substring::Substring;

fn main() {
    // get portnumber from user on start up
    let addr = local_ip().unwrap();
    println!("Enter port 1111, 2222 or 3333:");
    let mut port = String::new();
    match stdin().read_line(&mut port) {
        Ok(_) => {
            let port = port.trim();
            let port_ok = port == "1111" || port == "2222" || port == "3333";
            if port_ok {
                println!("Starting Onion-node with adress {}:{}", addr, port)
            } else {
                panic!("Port number not valid.")
            }
        },
        Err(e) => {
            panic!("Failed to read port: {}", e);
        }
    }
    // get private key from enum? based on nr
    let node_id = port.substring(0, 1).parse::<i32>().unwrap();
    let private_key = format!("temp key {}", node_id);
    // listen to port
    let full_addr = format!("127.0.0.1:{}", port.trim());

    let listener = TcpListener::bind(full_addr).expect("Could not establish listener!");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    // handle connection
                    handle_request(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

// handle connection:
fn handle_request(mut stream: TcpStream) {
    // decrypt layer
    // read header
    // pass on to destination
    // wait for response
    // encrypt response
    // pass response back to sender
}
    

    