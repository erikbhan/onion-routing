use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt};
use std::io::Write;

//use std::io::{Write, stdin, self};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

const N: usize = 3; //number of nodes, and therefore keys etc.

//util method; reads data from the user via stdin and returns immutable string
//message: a message can be shown to the user before their input
fn get_user_input(message: &str) -> &str {
    print!("{}", message);
    std::io::stdout().flush(); // Force print by flushing
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error when reading user input from stdin");
    input.trim()
}

//asks the DA for nodes; returns an array of nodes where [0] is the entry and [N] is the last
async fn get_nodes() -> [&'static str; N] {
    //let da_stream = TcpStream::connect(get_user_input("DA ADDR: "));
    
    //TODO: https is important here!!

    //TODO: client sends "can i haz n nodes" ->
    // gets n node-addresses over tls ->
    // parses adresses and returns method
    
    //currently just return node-addresses
    ["localhost:1111", "localhost:2222", "localhost:3333"]
}

async fn get_keys(nodes: [&str; N]) -> [&'static str; N] {
    ["k1", "k2", "k3"]
}

#[tokio::main]
async fn main() {
    //init client
    let nodes: [&str; N] = get_nodes().await;
    let keys: [&str; N] = get_keys(nodes).await;
    let mut connection = TcpStream::connect(nodes[0]).await;

    loop {
        let msg = get_user_input("Message: ");
        if msg.eq("exit") {
            break;
        };
        //msg.encrypt(keys);
        connection.write_all(msg.as_bytes()).await;
    }
    println!("Exiting program...");
}

// // TODO (Maybe): find path
// fn find_path() -> Vec<usize> {
//     let mut path = vec![0, 1, 2];

//     let mut rng = StepRng::new(2, 13);
//     let mut irs = Irs::default();
//     irs.shuffle(&mut path, &mut rng).expect("Could not shuffle path");

//     path
// }

/*
// TODO: Handshake nodes to get 
fn get_keys() {
    for i in 0..2 {
        // let key = sent_to_exchange_key_lib(NODES[i])
        // KEYS[i] = key
    }
}

// TODO: Write encrypting for path
fn encrypt(msg:&[u8], path:[i32; 3]) {
    let mut package = msg;
    for index in path.rev() {
        // set HEADER
            // destination = "localhost:{}", NODES[index]
        // encode package and header
            // private_key = KEYS[index]
            // send_to_encryption_lib(package, private_key)
    }
    return package;
}

// TODO: decrypt
fn decrypt(package:[u8;8], path:[i32; 3]) -> [u8;8] {
    let response;
    for index in path {
        // decode package
            // private_key = KEYS[index]
            // send_to_dencryption_lib(package, private_key)
    }
    return response;
}
*/

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