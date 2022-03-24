//use tokio::net::{TcpListener, TcpStream};
use openssl::pkey::PKey;
use openssl::rsa::{Rsa, Padding};
use openssl::aes::{AesKey, aes_ige};
use openssl::symm::Mode;
use openssl::rand::rand_bytes;
use hyper::{Client, Body, Method, Request, Uri};
use hyper_tls::HttpsConnector;

/* The node should work like this:
    1. On startup, a port should be chosen.
        Maybe randomly? Can be the same as other nodes if on different IP.
    2. When node is up and running, contact the DA. Send node's public key and address to DA over TLS.
    3. Ready and waiting for client connections.
*/

async fn send_public_key_to_da(aes_key: &[u8; 16]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let mut body = String::from(r#"{"library":""#);
    body.push_str(std::str::from_utf8(aes_key).unwrap());
    body.push_str(r#"hyper"}"#);
    let req = Request::builder()
        .method(Method::PUT)
        .uri("localhost:5000") // TODO: DA's address
        .header("content-type", "application/json")
        .body(Body::from(body))?;

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let res = client.request(req).await?;

    println!("Response: {}", res.status());

    Ok(())
}

async fn prepare_keys() -> AesKey {
    let mut buf = [0; 16];
    rand_bytes(&mut buf).unwrap();
    let aes_key = AesKey::new_encrypt(&buf).unwrap();
    let mut iv = *b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\
        \x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";
    send_public_key_to_da(&buf).await;
    aes_key
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let aes_key = prepare_keys();

    let client = Client::new();
    let uri = "http://httpbin.org/ip";
    let res = client.get(Uri::from_static(uri)).await.unwrap();

    println!("Response: {}", res.status());
    Ok(())


    
    /*
    // get portnumber from user on start up
    //let addr = local_ip().unwrap();
    //println!("Enter port 1111, 2222 or 3333:");
    /*let mut port = String::new();
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
    }*/
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
    }*/
}

// handle connection:
//fn handle_request(mut stream: TcpStream) {
    // decrypt layer
    // read header
    // pass on to destination
    // wait for response
    // encrypt response
    // pass response back to sender
//}
    

    