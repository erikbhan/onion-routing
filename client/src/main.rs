use std::error::Error;
use std::io::{Write};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};

use native_tls::TlsConnector;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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
    
    parse_array(parsable_string)
}

fn parse_array(parsable_string:String) -> Vec<String> {
    let split:Vec<&str> = parsable_string.split(", ").collect();
    let mut array:Vec<String> = [].to_vec();
    for string in split {
        array.append(&mut [string.to_string()].to_vec());
    }
    array
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let nodes = request_from_da("nodes").await;
    let keys = request_from_da("keys").await;

    //let mut connection = TcpStream::connect("127.0.0.1:8080").await?; //REMEMBER: update addr here to entry node when implemented

    loop {
        let msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        };
        println!("msg: '{}'", msg);

        let enc_data = encrypt_message(msg, keys.clone());
        println!("enc_data: '{:?}'", enc_data);

        let dec_data = decrypt_message(enc_data, keys.clone());
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

#[cfg(test)]
mod client_test {
    use super::*;

    #[test]
    fn read_message_into_buffer_test() {
        assert!(false);
    }

    #[test]
    fn parse_array_test() {
        assert_eq!(parse_array("1, 2, 3".to_string()), ["1".to_owned(), "2".to_owned(), "3".to_owned()].to_vec())
    }
    
    #[test]
    fn request_from_da_test() {
        assert!(false);
    }

    #[test]
    fn get_user_input_test() {
        assert!(false);
    }
}