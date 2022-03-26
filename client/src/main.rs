use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::io::{Write, Read, stdout};
use native_tls::{ TlsConnector };
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

const N: usize = 3; //number of nodes, and therefore keys etc.
const DA_ADDR: &str = "0.0.0.0";
const DA_PORT: &str = "8443";

//util method; reads data from the user via stdin and returns immutable string
//message: a message can be shown to the user before their input
fn get_user_input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush().expect("Not all bytes could be written to terminal"); // Force print by flushing
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error when reading user input from stdin");
    input.trim().to_string()
}

//asks the DA for nodes; returns an array of nodes where [0] is the entry and [N] is the last
async fn request_from_da(nodes_or_keys:&str) -> [String;N] {
    let rec = format!("GET {} HTTPS/1.1", nodes_or_keys);
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build().expect("Error when building TLS connection");

    let stream:std::net::TcpStream = std::net::TcpStream::connect(format!("{}:{}", DA_ADDR, DA_PORT)).unwrap();

    // Domain will be ignored since cert/hostname verification is disabled
    let mut stream = connector.connect(DA_ADDR, stream).unwrap();

    stream.write_all(rec.as_bytes()).unwrap();
    let mut res = vec![];

    stream.read_to_end(&mut res).expect("Failed to receive live nodes and keys from DA");
    
    let parsable_string = String::from_utf8(res.to_vec()).unwrap();
    
    parse_array(parsable_string)
}

fn parse_array(parsable_string: String) -> [String; N] {
    let split: Vec<&str> = parsable_string.split(", ").collect();
    [split[0].to_owned(), split[1].to_owned(), split[2].to_owned()]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let message = String::from("Hello world!");
    let keys = [String::from("AAAABBBBCCCCDDDD"), String::from("AAAABBBBDDDDDDDD"), String::from("AAAAAAAACCCCDDDD")];
    let encrypted = encrypt_message(message, keys.clone());
    let decrypted = decrypt_message(encrypted, keys);
    
    // let nodes: [String; N] = request_from_da("nodes").await;
    // let keys: [String; N] = request_from_da("keys").await;

    // print!("{:?}", nodes);
    // print!("{:?}", keys);

    // let mut connection = TcpStream::connect("127.0.0.1:8080").await?; //REMEMBER: update addr here to entry node when implemented

    // loop {
    //     let msg = get_user_input("Message: ");
    //     if msg.eq("exit") {
    //         break;
    //     };

    //     connection.write_all(msg.as_bytes()).await?;
    //     let buf = read_message_into_buffer(&connection).await;
    //     let response = String::from_utf8(buf.to_vec()).unwrap();
    //     print!("{}", response);
    // }
    // println!("Exiting program...");
    Ok(())
}

async fn read_message_into_buffer(stream: &TcpStream) -> [u8; 4096] {
    stream.readable().await.expect("Could not read buffer from server");
    let mut buf = [0u8; 4096];
    match stream.try_read(&mut buf) {
        Ok(0) => {
            println!("Stream closed");
            buf
        }
        Ok(n) => {
            println!("read {} bytes", n);
            buf
        }
        Err(e) => {
            print!("An errror occured when recieving respose from server: {}", e);
            [0u8; 4096]
        }
    }
}

fn encrypt_message(plaintext: String, keys: [String; N]) -> Vec<u8> {
    let mut keys = keys;
    keys.reverse();

    let key1 = key_from_string(keys[0].clone());
    let key2 = key_from_string(keys[1].clone());
    let key3 = key_from_string(keys[2].clone());

    let key1 = Key::from_slice(&key1);
    let key2 = Key::from_slice(&key2);
    let key3 = Key::from_slice(&key3);

    let cipher1 = Aes256Gcm::new(key1);
    let cipher2 = Aes256Gcm::new(key2);
    let cipher3 = Aes256Gcm::new(key3);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    println!("Plaintext: '{}', as bytes: {:?}", plaintext, plaintext.as_bytes());
    let ciphertext = cipher1.encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Encryption 1: {:?}", ciphertext);
    let ciphertext = cipher2.encrypt(nonce, ciphertext.as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Encryption 2: {:?}", ciphertext);
    let ciphertext = cipher3.encrypt(nonce, ciphertext.as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Encryption 3: {:?}", ciphertext);
    
    ciphertext
    // let mut keys = keys;
    // keys.reverse();
    // let ciphertext = plaintext.into_bytes();
    // for key_str in keys {
    //     let key = key_from_string(key_str.clone());
    //     let key = Key::from(key);
    //     let cipher = Aes256Gcm::new(&key);
    //     let nonce = Nonce::from_slice(b"unique nonce"); //96-bits, unique per message
    //     let ciphertext = cipher.encrypt(nonce, ciphertext.as_ref()).expect("Error during encryption");  
    // }
    // ciphertext
}

fn decrypt_message(ciphertext: Vec<u8>, keys: [String; N]) -> Vec<u8> {
    let key1 = key_from_string(keys[0].clone());
    let key2 = key_from_string(keys[1].clone());
    let key3 = key_from_string(keys[2].clone());

    let key1 = Key::from_slice(&key1);
    let key2 = Key::from_slice(&key2);
    let key3 = Key::from_slice(&key3);

    let cipher1 = Aes256Gcm::new(key1);
    let cipher2 = Aes256Gcm::new(key2);
    let cipher3 = Aes256Gcm::new(key3);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    println!("Ciphertext: {:?}", ciphertext);
    let ciphertext = cipher1.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Decryption 1: {:?}", ciphertext);
    let ciphertext = cipher2.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Decryption 2: {:?}", ciphertext);
    let plaintext = cipher3.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    println!("Decryption 3: {:?}", plaintext);
    
    // let plaintext = vec![0u8];
    // for key_str in keys {
    //     let key = key_from_string(key_str.clone());
    //     let key = Key::from(key);
    //     let cipher = Aes256Gcm::new(&key);
    //     let nonce = Nonce::from_slice(b"unique nonce"); //96-bits, unique per message
    //     let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).expect("Error during decryption");  
    // }
    plaintext
}

fn key_from_string(key_as_string: String) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[0..key_as_string.len()].copy_from_slice(key_as_string.as_bytes());
    bytes
}