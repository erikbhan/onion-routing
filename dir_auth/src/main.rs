extern crate native_tls;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut file = File::open("foo.p12").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Identity::from_pkcs12(&pkcs12, "").unwrap();

    let acceptor = TlsAcceptor::new(pkcs12).unwrap();
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

    // Type alias?
    let nodes = Arc::new(Mutex::new(Vec::new()));
    let keys = Arc::new(Mutex::new(Vec::new()));

    fn handle_client(mut stream: TlsStream<TcpStream>, nodes_clone: Arc<Mutex<Vec<String>>>, keys_clone: Arc<Mutex<Vec<String>>>) {
        let mut buf = [0u8; 4096];
        let num_bytes_read = stream.read(&mut buf).unwrap();
        let data = String::from_utf8(buf[0..num_bytes_read].to_vec()).unwrap();
        let peer_addr = stream.get_ref().peer_addr().unwrap();
        
        let mut nodes = nodes_clone.lock().unwrap();
        let mut keys = keys_clone.lock().unwrap();
        nodes.push(peer_addr.to_string());
        keys.push(data);
        println!("Pushed {} to nodes", peer_addr);

        // Answer incoming stream with ok
        stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
        stream.shutdown();
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let nodes_clone = Arc::clone(&nodes);
                let keys_clone = Arc::clone(&keys);
                thread::spawn(
                    move || {
                        let stream = acceptor.accept(stream).unwrap();
                        handle_client(stream, nodes_clone, keys_clone);
                    }
                );
            }
            Err(_e) => { /* connection failed */ }
        }
        println!("{:?}", nodes);
    }
}