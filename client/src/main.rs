use openssl::error::ErrorStack;
use openssl::rsa::Rsa;
// Async/await
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

// STD
use std::error::Error;
use std::io::Write;

// Cryptography
use openssl::dh::Dh;
use openssl::pkey::{PKey, Private};

/// Number of nodes/keys to use and get from the DA. (Put in config file?)
const N: usize = 3;

// TODO: Implement.
async fn get_nodes() -> [&'static str; N] {
    ["localhost:1111", "localhost:2222", "localhost:3333"]
}

// TODO: Implement. Returns array of shared keys with nodes.
async fn get_keys(nodes: [&str; N]) -> [Result<PKey<Private>, ErrorStack>; N] {
    let rsa1 = Rsa::generate(2048).unwrap();
    let rsa2 = Rsa::generate(2048).unwrap();
    let rsa3 = Rsa::generate(2048).unwrap();
    let pkey1 = PKey::from_rsa(rsa1);
    let pkey2 = PKey::from_rsa(rsa2);
    let pkey3 = PKey::from_rsa(rsa3);
    [pkey1, pkey2, pkey3]
}

fn encrypt(data: &str, keys: [PKey<&str>; N]) -> String {
    let encrypted_data = String::new();
    for i in (0..N-1).rev() {

    }
    "encrypted_data".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let nodes: [&str; N] = get_nodes().await;
    //let keys: [&str; N] = get_keys(nodes).await;

    // TODO: connect to entry node, not localhost
    let mut connection = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        let msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        };
        //encrypt(msg, keys);
        connection.write_all(&msg.as_bytes()).await?;
        let buf = read_message_into_buffer(&connection).await;
    }
    println!("Exiting program...");
    Ok(())
}

// // TODO: Write encrypting for path
// fn encrypt(msg:&[u8], path:[i32; 3]) {
//     let mut package = msg;
//     for index in path.rev() {
//         // set HEADER
//             // destination = "localhost:{}", NODES[index]
//         // encode package and header
//             // private_key = KEYS[index]
//             // send_to_encryption_lib(package, private_key)
//     }
//     return package;
// }

// // TODO: decrypt
// fn decrypt(package:[u8;8], path:[i32; 3]) -> [u8;8] {
//     let response;
//     for index in path {
//         // decode package
//             // private_key = KEYS[index]
//             // send_to_dencryption_lib(package, private_key)
//     }
//     return response;
// }

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
        //todo: error handling 😇👼
        // Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        //     continue;
        // }
        Err(e) => {
            //return Err(e.into());
            return buf;
        }
    }
}

// TODO: send to first node in path
// fn send(msg:&[u8], node:usize) {
//     let addr = format!("localhost:{}", NODES[node]);
//     match TcpStream::connect(addr) {
//         Ok(mut stream) => {
//             stream.write_all(msg).unwrap();
//             println!("Sent, awaiting reply...");

//             // recieve answer:
//             let mut data = [0_u8; 8];
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     let result = data; // decrypt(data);
//                     println!("Recieved: {:?}", String::from_utf8(result.to_vec()));
//                 },
//                 Err(e) => {
//                     println!("Failed to receive data: {}", e);
//                 }
//             }
//         },
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
// }

/// Returns a String given by the user through stdin. Optional message prompt.
fn get_user_input(message: &str) -> String {
    print!("{}", message);
    std::io::stdout().flush(); // Force print by flushing
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error when reading user input from stdin");
    input.trim().to_string()
}