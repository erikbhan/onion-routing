use std::error::Error;
use std::io::{Write};
use aes_gcm::aead::consts::{B1, B0};
use aes_gcm::aead::generic_array::typenum::UInt;
use aes_gcm::{Key, Nonce, AesGcm, Aes256Gcm}; // Or `Aes128Gcm`
use aes_gcm::aead::{NewAead, Aead};

use native_tls::TlsConnector;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const DA_ADDR: &str = "0.0.0.0";
const DA_PORT: &str = "8443";
const N:usize = 3;

//util method; reads data from the user via stdin and returns immutable string
//message: a message can be shown to the user before their input
fn get_user_input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush().expect("Not all bytes could be written to terminal"); // Force print by flushing
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or_default(); // or default should return nothing that is caught in the main loop
    input.trim().to_string()
}

//asks the DA for nodes; returns an array of nodes where [0] is the entry and [N] is the last
async fn request_from_da(nodes_or_keys:&str) -> Vec<String> {
    let rec = format!("GET {} HTTPS/1.1", nodes_or_keys);

    let stream = TcpStream::connect(format!("{}:{}", DA_ADDR, DA_PORT)).await.unwrap();
    let cx = TlsConnector::builder()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true)
    .build().expect("Error when creating TLS connector builder");
    let cx = tokio_native_tls::TlsConnector::from(cx);
    let mut stream = cx.connect(DA_ADDR, stream).await.unwrap();

    stream.write_all(rec.as_bytes()).await.expect("Error when sending to DA");
    let mut data = Vec::new();
    stream.read_to_end(&mut data).await.expect("Error when reading from DA");
    
    let parsable_string = String::from_utf8(data).unwrap();
    
    let mut vec = parse_array(parsable_string);
    if vec.len() > N {
        vec = vec[0..N].to_vec();
    }
    vec
}

fn parse_array(parsable_string:String) -> Vec<String> {
    let split:Vec<&str> = parsable_string.split(", ").collect();
    let mut array:Vec<String> = [].to_vec();
    for string in split {
        array.push(string.to_string());
    }
    array
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let nodes = request_from_da("nodes").await;
    let keys_unformatted = request_from_da("keys").await;
    let keys = format_keys(keys_unformatted);

    //let mut connection = TcpStream::connect("127.0.0.1:8080").await?; //REMEMBER: update addr here to entry node when implemented

    loop {
        let msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        }
        if msg.is_empty() { // if message from user is empty the loop will try again
            print!("Input was empty (possibly due to an error), please try again.");
            continue;
        }
        println!("msg: '{}'", msg);

        let mut enc_data = Vec::new(); 
        match encrypt_message(msg, keys.clone()) {
            Ok(val) => {
                enc_data = val;
            }
            Err(err) => {
                // maybe panic here?
                print!("Could not encrypt message due to unforseen error: {}", err);
            }
        } 
        println!("enc_data: '{:?}'", enc_data);

        let mut dec_data = Vec::new();
        match decrypt_message(enc_data, keys.clone()) {
            Ok(val) => {
                dec_data = val;
            }
            Err(err) => {
                // maybe panic here?
                print!("Could not encrypt message due to unforseen error: {}", err);
            }
        };
        println!("dec_data: '{:?}'", dec_data);

        //connection.write_all(&enc_data).await?;
        //let buf = read_message_into_buffer(&connection).await;
        //let dec_data = decrypt_message(buf.to_vec(), keys.clone());
        //let response = String::from_utf8(dec_data).unwrap();
        //print!("{}", response);
    }
    println!("Exiting program...");
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

fn encrypt_message(plaintext: String, keys: Vec<[u8;32]>) -> Result<Vec<u8>, aes_gcm::Error>  {
    let mut keys = keys;
    keys.reverse();

    let ciphers = generate_ciphers(keys);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let mut ciphertext = ciphers[0].encrypt(nonce, plaintext.as_bytes());
    for i in 1..N {
        ciphertext = ciphers[i].encrypt(nonce, ciphertext.unwrap().as_ref());
    }
    ciphertext
}

fn decrypt_message(ciphertext: Vec<u8>, keys: Vec<[u8;32]>) -> Result<Vec<u8>, aes_gcm::Error>  {
    let mut keys = keys;

    let ciphers = generate_ciphers(keys);
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let mut ciphertext:Result<std::vec::Vec<u8>, aes_gcm::Error> = Ok(ciphertext);
    for i in 0..N {
        ciphertext = ciphers[i].decrypt(nonce, ciphertext.unwrap().as_ref());
    }
    ciphertext
}

fn generate_ciphers(keys: Vec<[u8;32]>) -> Vec<AesGcm<aes_gcm::aes::Aes256, UInt<UInt<UInt<UInt<aes_gcm::aead::generic_array::typenum::UTerm, B1>, B1>, B0>, B0>>>{
    let mut ciphers = Vec::with_capacity(N);
    for key in keys {
        let key1 = Key::from_slice(&key);
        let cipher1 = Aes256Gcm::new(key1);
        ciphers.push(cipher1);
    }
    ciphers
}

fn format_keys(keys: Vec<String>) -> Vec<[u8; 32]> {
    let mut formatted_keys:Vec<[u8; 32]> = Vec::new();
    for key in keys {
        let key_from_string = key_from_string(key);
        formatted_keys.append(&mut [key_from_string].to_vec());
    }
    formatted_keys
}

fn key_from_string(key_as_string: String) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[0..key_as_string.len()].copy_from_slice(key_as_string.as_bytes());
    bytes
}

#[cfg(test)]
mod client_test {
    use super::*;

    /*
    #[tokio::test]
    async fn read_message_into_buffer_test() {
        assert!(false);
    }
    */

    #[test]
    fn parse_array_test() {
        assert_eq!(parse_array("1, 2, 3".to_string()), ["1".to_owned(), "2".to_owned(), "3".to_owned()].to_vec())
    }

    // This test passes if the user writes great, but needs to be ignored for cicd
    #[test]
    #[ignore = "Testing user-input from teminal requiers adding lots of unnessesary code to support mocking"]
    fn get_user_input_test() {
        assert_eq!(get_user_input("How are you?"), "great");
    }

    #[test]
    fn encrypt_message_test() {
        let keys_unformatted = ["Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string()];
        let keys = format_keys(keys_unformatted.to_vec());
        let mut key_copy = keys.to_owned();

        let encrypted = encrypt_message("plaintext".to_string(), keys).expect("This test fails if the method panics");

        key_copy.reverse();

        let key1 = Key::from_slice(&key_copy[0]);
        let key2 = Key::from_slice(&key_copy[1]);
        let key3 = Key::from_slice(&key_copy[2]);

        let cipher1 = Aes256Gcm::new(key1);
        let cipher2 = Aes256Gcm::new(key2);
        let cipher3 = Aes256Gcm::new(key3);

        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher1.encrypt(nonce, "plaintext".as_bytes())
            .unwrap();
        let ciphertext = cipher2.encrypt(nonce, ciphertext.as_ref())
            .unwrap();
        let ciphertext = cipher3.encrypt(nonce, ciphertext.as_ref())
            .unwrap();
        
        assert_eq!(ciphertext, encrypted)
    }

    #[test]
    fn decrypt_message_test() {
        let keys_unformatted = ["Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string()];
        let keys = format_keys(keys_unformatted.to_vec());
        let key_copy = keys.to_owned();
        let key_copy2 = keys.to_owned();
        let encrypted = encrypt_message("plaintext".to_string(), keys).expect("This test fails if the method panics");
        let encrypted_copy = encrypted.to_owned();

        let decrypted = decrypt_message(encrypted, key_copy2).expect("This test fails if the method panics");

        let key1 = Key::from_slice(&key_copy[0]);
        let key2 = Key::from_slice(&key_copy[1]);
        let key3 = Key::from_slice(&key_copy[2]);

        let cipher1 = Aes256Gcm::new(key1);
        let cipher2 = Aes256Gcm::new(key2);
        let cipher3 = Aes256Gcm::new(key3);

        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher1.decrypt(nonce, encrypted_copy.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
        let ciphertext = cipher2.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
        let plaintext = cipher3.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
        
        assert_eq!(plaintext, decrypted)
    }

    #[test]
    fn format_keys_test() {
        let keys_unformatted = ["Writing tests is slow and boring".to_string(), "Writing tests is slow and boring".to_string(), "Writing tests is slow and boring".to_string()];
        let keys_formatted = format_keys(keys_unformatted.to_vec());
        let keys = [b"Writing tests is slow and boring".to_owned(), b"Writing tests is slow and boring".to_owned(), b"Writing tests is slow and boring".to_owned()].to_vec();

        assert_eq!(keys_formatted, keys)
    }

    #[test]
    fn key_from_string_test() {
        let key_string = "Writing tests is slow and boring";
        let key = b"Writing tests is slow and boring";
        let key_from_string = key_from_string(key_string.to_string());

        assert_eq!(key.to_owned(), key_from_string)
    }
}