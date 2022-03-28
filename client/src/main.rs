use std::error::Error;
use std::io::{Write};
use std::vec;

use aes_gcm::aead::consts::{B1, B0};
use aes_gcm::aead::generic_array::typenum::UInt;
use aes_gcm::{Key, Nonce, AesGcm, Aes256Gcm}; // Or `Aes128Gcm`
use aes_gcm::aead::{NewAead, Aead};

use native_tls::TlsConnector;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

type Cipher = Vec<AesGcm<aes_gcm::aes::Aes256, UInt<UInt<UInt<UInt<aes_gcm::aead::generic_array::typenum::UTerm, B1>, B1>, B0>, B0>>>;
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
async fn request_from_da() -> (Vec<String>, Vec<String>) {
    let rec = "GET /nodes HTTPS/1.1".to_string();

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
    
    parse_array(parsable_string)
}

fn parse_array(parsable_string:String) -> (Vec<String>, Vec<String>) {
    let split:Vec<&str> = parsable_string.split('\\').collect();
    let mut nodes:Vec<String> = Vec::with_capacity(3);
    let mut keys:Vec<String> = Vec::with_capacity(3);
    for string in split {
        if !string.contains("node") {
            continue;
        }
        let (node, key) = parse_node(string);
        nodes.push(node);
        keys.push(key);
    }

    (nodes, keys)
}

fn parse_node(str:&str) -> (String, String){
    // "node: 1, key: 1"
    let split:Vec<&str> = str.split(", ").collect();
    let node_addr = split[0].to_owned()[6..].to_string();
    let key = split[1].to_owned()[5..].to_string();

    (node_addr, key)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (nodes,keys_unformatted) = request_from_da().await;
    let keys = format_keys(keys_unformatted);

    //let mut connection = TcpStream::connect("127.0.0.1:8080").await?; //REMEMBER: update addr here to entry node when implemented

    loop {
        let mut msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        } else if msg.is_empty() {
            println!("Input was empty, please try again.");
            continue;
        }

        let mut enc_data = Vec::new();
        //Stop using .clone on keys and nodes
        match encrypt_message(&mut msg, keys.clone(), nodes.clone()) {
            Ok(val) => {
                enc_data = val;
            }
            Err(err) => {
                // maybe panic here?
                print!("Could not encrypt message due to unforseen error: {}", err);
            }
        } 

        //connection.write_all(&enc_data).await?;
        //let buf = read_message_into_buffer(&connection).await;
        //let dec_data = decrypt_message(buf.to_vec(), keys.clone());
        //let response = String::from_utf8(dec_data).unwrap();
        //print!("{}", response);

        let mut dec_data = Vec::new();
        match decrypt_message(enc_data, keys.clone()) {
            Ok(val) => {
                dec_data = val;
            }
            Err(err) => {
                println!("Could not decrypt message: {}", err); // Should we panic?
            }
        };

        println!("{:?}", String::from_utf8_lossy(&dec_data));
    }
    println!("Exiting program...");
    Ok(())
}

fn encrypt_message(plaintext: &mut String, keys: Vec<[u8;32]>, nodes: Vec<String>) -> Result<Vec<u8>, aes_gcm::Error>  {
    let mut keys = keys;
    let mut nodes = nodes;
    let mut nonces = vec![Nonce::from_slice(b"uniqua nonce"), Nonce::from_slice(b"unidue nonce"), Nonce::from_slice(b"ucique nonce")];
    keys.reverse();
    nodes.reverse();
    nonces.reverse();

    let ciphers = generate_ciphers(keys);
    
    plaintext.push_str(&format!("\\{}\\", &nodes[0]));
    // println!();
    // println!("Decrypted: {}", &plaintext);
    let mut ciphertext = ciphers[0].encrypt(nonces[0], plaintext.as_bytes());
    match ciphertext.clone() {
        Ok(_value) => {
            // println!("Encrypted: {}", String::from_utf8_lossy(&value));
        },
        Err(err) => {
            println!("{}", err)
        }
    }


    for i in 1..N {
        // println!();
        let mut unwrapper_ciphertext = vec![];
        match ciphertext {
            Ok(text) => {
                //println!("Decrypted: {}", String::from_utf8_lossy(&text));
                unwrapper_ciphertext = text;
            },
            Err(err) => {
                println!("{}", err);
            }
        }
        let ip_str = format!("\\{}\\", &nodes[i]);
        let mut ip = ip_str.into_bytes();
        unwrapper_ciphertext.append(&mut ip);
        // println!("Decrypted: {}", String::from_utf8_lossy(&unwrapper_ciphertext));

        ciphertext = ciphers[i].encrypt(nonces[i], unwrapper_ciphertext.as_ref());
        match ciphertext.clone() {
            Ok(_val2) => {
                // println!("Encrypted: {}", String::from_utf8_lossy(&val2));
                //decrypted = val2;
            },
            Err(err) => {
                println!("{}", err);
            }
        }

    }
    ciphertext
}

fn decrypt_message(ciphertext: Vec<u8>, keys: Vec<[u8;32]>) -> Result<Vec<u8>, aes_gcm::Error>  {
    let keys = keys;

    let ciphers = generate_ciphers(keys);
    let nonces = vec![Nonce::from_slice(b"uniqua nonce"), Nonce::from_slice(b"unidue nonce"), Nonce::from_slice(b"ucique nonce")];
    //let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message


    let mut plaintext = Ok(ciphertext);
    for i in 0..N {

        let mut text = vec![];
        match plaintext {
            Ok(val) => {
                // println!();
                // println!("Encrypted: {}", String::from_utf8_lossy(&val));
                text = val;
            },
            Err(err) => {
                println!("{}", err);
            }
        }

        let mut ip_removed = text;
        if i != 0 {
            ip_removed = ip_removed[..ip_removed.len()-16].to_vec();
        }
        plaintext = ciphers[i].decrypt(nonces[i], ip_removed.as_ref());

        //let mut decrypted = vec![];
        match plaintext.clone() {
            Ok(_val2) => {
                // println!("Decrypted: {}", String::from_utf8_lossy(&val2));
                //decrypted = val2;
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
    plaintext
}

fn generate_ciphers(keys: Vec<[u8;32]>) -> Cipher {
    let mut ciphers = Vec::with_capacity(N);
    for key in keys {
        let key_from_slice = Key::from_slice(&key);
        let cipher = Aes256Gcm::new(key_from_slice);
        ciphers.push(cipher);
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
mod test;