extern crate native_tls;

use native_tls::{ TlsConnector };
use std::io::{Read, Write};
use std::net::TcpStream;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

fn init() {
    let key = b"an example very very secret key."; //32 byte
    
    // TODO: Error handling
    // TODO: Cert verification    
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build().unwrap();

    let stream = TcpStream::connect("localhost:8443").unwrap();

    // Domain will be ignored since cert/hostname verification is disabled
    let mut stream = connector.connect("localhost", stream).unwrap();

    stream.write_all(key).unwrap();
    let mut res = vec![];

    // TODO: Error if not 200 OK
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

fn main() {
    init();
    // TODO: Find better way to handle key (generate on startup).
    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);

    // TODO: Gen new nonce for every message with a cryptographically secure random byte generator
    // TODO: Nonce has to be stored; it is needed for decryption (I think?).
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

    assert_eq!(&plaintext, b"plaintext message");
    println!("{}", std::str::from_utf8(&plaintext).unwrap());
}



/*
async fn send_public_key_to_da(aes_key: &[u8; 16]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let mut body = String::from(r#"{"library":""#);
    body.push_str(std::str::from_utf8(aes_key).unwrap());
    body.push_str(r#"hyper"}"#);
    let req = Request::builder()
        .method(Method::PUT)
        .uri("localhost:8443") // TODO: DA's address
        .header("content-type", "application/json")
        .body(Body::from(body))?;

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let res = client.request(req).await?;

    println!("Response: {}", res.status());

    Ok(())
}

async fn prepare_keys() -> &'static str {
    let buf = b"ABCDABCDABCDABCD";
    //let mut buf = [0u8; 16];
    //let mut iv = [0u8; 16];
    //rand_bytes(&mut buf).unwrap();
    //rand_bytes(&mut iv).unwrap();
    //let aes_key = AesKey::new_encrypt(&buf).unwrap();
    send_public_key_to_da(buf).await;
    std::str::from_utf8(buf).unwrap()
}

#[tokio::main]
async fn main() {
    let aes_key = prepare_keys().await;

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
    

    */