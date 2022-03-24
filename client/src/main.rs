use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::io::{Write, Read};
use native_tls::{ TlsConnector };
use serde_json::{Value, json};

const N: usize = 3; //number of nodes, and therefore keys etc.
const DA_ADDR: &str = "0.0.0.0";
const DA_PORT: &str = "8443";

//util method; reads data from the user via stdin and returns immutable string
//message: a message can be shown to the user before their input
fn get_user_input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush(); // Force print by flushing
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error when reading user input from stdin");
    input.trim().to_string()
}

//asks the DA for nodes; returns an array of nodes where [0] is the entry and [N] is the last
async fn get_nodes_and_keys(mut keys: [&str;N], mut nodes: [&str;N]) {
    let rec = b"GET /nodes/keys HTTPS/1.1";
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build().expect("Error when building TLS connection");

    let stream:std::net::TcpStream = std::net::TcpStream::connect(format!("{}:{}", DA_ADDR, DA_PORT)).unwrap();

    // Domain will be ignored since cert/hostname verification is disabled
    let mut stream = connector.connect(DA_ADDR, stream).unwrap();

    stream.write_all(rec).unwrap();
    let mut res = vec![];

    stream.read_to_end(&mut res).expect("Failed to receive live nodes and keys from DA");
    
    let parsable_string = String::from_utf8(res.to_vec()).unwrap();
    let jsonValue:Value = json!(parsable_string.as_str());

    nodes = jsonValue["nodes"];
    keys = *jsonValue.get("keys");

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut nodes: [&str; N];
    let mut keys: [&str; N];

    get_nodes_and_keys(keys, nodes).await;

    let mut connection = TcpStream::connect("127.0.0.1:8080").await?; //REMEMBER: update addr here to entry node when implemented

    loop {
        let msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        };

        connection.write_all(&msg.as_bytes()).await?;
        let buf = read_message_into_buffer(&connection).await;
        let response = String::from_utf8(buf.to_vec()).unwrap();
        print!("{}", response);
    }
    println!("Exiting program...");
    Ok(())
}

async fn read_message_into_buffer(stream: &TcpStream) -> [u8; 4096] {
    stream.readable().await;
    let mut buf = [0u8; 4096];
    match stream.try_read(&mut buf) {
        Ok(0) => {
            println!("Stream closed");
            return buf;
        }
        Ok(n) => {
            println!("read {} bytes", n);
            return buf;
        }
        Err(e) => {
            print!("An errror occured when recieving respose from server: {}", e.to_string());
            let empty_buf = [0u8; 4096];
            return empty_buf;
        }
    }
}

