mod node_key;

extern crate native_tls;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use node_key::NodeKey;

fn main() {
    let mut file = File::open("foo.p12").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Identity::from_pkcs12(&pkcs12, "").unwrap();

    let acceptor = TlsAcceptor::new(pkcs12).unwrap();
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

    // Type alias?
    let node_vec:Vec<NodeKey> = Vec::new();
    let nodes = Arc::new(Mutex::new(node_vec));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let nodes_clone = Arc::clone(&nodes);
                thread::spawn(
                    move || {
                        let stream = acceptor.accept(stream).unwrap();
                        handle_client(stream, nodes_clone);
                    }
                );
            }
            Err(e) => { 
                print!("An error occured when receiving requests from node or client: {}", e);
            }
        }
        
        println!("{:?}", nodes);
    }
}

fn handle_client(mut stream: TlsStream<TcpStream>, nodes_clone: Arc<Mutex<Vec<NodeKey>>>) {
    let mut buf = [0u8; 4096];
    let num_bytes_read = stream.read(&mut buf).unwrap();
    let data = String::from_utf8(buf[0..num_bytes_read].to_vec()).unwrap();
    let peer_addr = stream.get_ref().peer_addr().unwrap();

    let mut nodes = nodes_clone.lock().unwrap();

    // Handle request for nodes from client
    if data.contains("GET")  {
        stream.write_all(get_nodes_and_keys(nodes).as_bytes()).unwrap();
        stream.shutdown().expect("Stream shutdown returned an error");
        return;
    }
    
    let node_key = NodeKey{node:peer_addr.to_string(), key: data};
    nodes.push(node_key);
    println!("Pushed {} to nodes", peer_addr);

    // Answer incoming stream with ok
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.shutdown().expect("Stream shutdown returned an error");
}

fn get_nodes_and_keys(nodes:MutexGuard<Vec<NodeKey>>) -> String {
    let index = get_random_path(nodes.len());

    let mut send_string = String::new();

    for i in index {
        let random_node_key = &nodes[i].to_string();
        send_string += &format!("\\{}\\", random_node_key);
    }

    send_string
}

fn get_random_path(len: usize) -> [usize;3] {
    let mut rng = rand::thread_rng();
    let mut all_indexes = Vec::with_capacity(len);

    for i in 0..len {
        all_indexes.push(i);
    }

    let mut index:[usize;3] = [0, 1, 2];

    (0..3).for_each(|i| {
        let rand:usize = rng.gen_range(0..len-i);
        index[i] = all_indexes.remove(rand);
    });

    index
}

#[cfg(test)]
mod test;