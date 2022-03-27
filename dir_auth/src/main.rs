extern crate native_tls;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
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
            Err(e) => { 
                print!("An error occured when receiving requests from node or client: {}", e);
            }
        }
        println!("{:?}", nodes);
    }
}

fn handle_client(mut stream: TlsStream<TcpStream>, nodes_clone: Arc<Mutex<Vec<String>>>, keys_clone: Arc<Mutex<Vec<String>>>) {
    let mut buf = [0u8; 4096];
    let num_bytes_read = stream.read(&mut buf).unwrap();
    let data = String::from_utf8(buf[0..num_bytes_read].to_vec()).unwrap();
    let peer_addr = stream.get_ref().peer_addr().unwrap();

    let mut nodes = nodes_clone.lock().unwrap();
    let mut keys = keys_clone.lock().unwrap();

    // Handle request for nodes from client
    if data.contains("GET")  {
        // !!!!! TESTING KEYS !!!!! -- remove when nodes work
        keys.push("an example very very secret key.".to_string());
        keys.push("an example very very secret key.".to_string());
        keys.push("an example very very secret key.".to_string());
        // !!!!! TESTING KEYS !!!!!
        stream.write_all(get_nodes_or_keys(data, nodes, keys).as_bytes()).unwrap();
        stream.shutdown().expect("Stream shutdown returned an error");
        return;
    }
    
    nodes.push(peer_addr.to_string());
    keys.push(data);
    println!("Pushed {} to nodes", peer_addr);

    // Answer incoming stream with ok
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.shutdown().expect("Stream shutdown returned an error");
}

fn get_nodes_or_keys(data:String, nodes:MutexGuard<Vec<String>>, keys:MutexGuard<Vec<String>>) -> String {
    let mut send_string;
    if data.eq("GET nodes HTTPS/1.1") {
        let mut iter = nodes.iter().cloned();
        send_string = iter.next().unwrap();
        for _i in 1..nodes.len() {
            send_string += &(", ".to_string() + &iter.next().unwrap());
        }
    }
    else if data.eq("GET keys HTTPS/1.1") {
        let mut iter = keys.iter().cloned();
        send_string = iter.next().unwrap();
        for _i in 1..keys.len() {
            send_string += &(", ".to_string() + &iter.next().unwrap());
        }
    } 
    else {
        send_string = "GET request not in the right format.".to_string();
    }
    send_string
}

#[cfg(test)]
mod dir_auth_test {
    use super::*;

    #[test]
    fn get_nodes_or_keys_test() {
        // test prerequisites:
        let test_vec_node = ["1".to_string(), "2".to_string(), "3".to_string()].to_vec();
        let test_vec_key = ["42".to_string(), "42".to_string(), "42".to_string()].to_vec();
        let expected_sting_node = "1, 2, 3";
        let expected_sting_key = "42, 42, 42";
        let nodes = Arc::new(Mutex::new(test_vec_node));
        let keys = Arc::new(Mutex::new(test_vec_key));

        // get nodes:
        assert_eq!(get_nodes_or_keys("GET nodes HTTPS/1.1".to_string(), nodes.lock().unwrap(), keys.lock().unwrap()), expected_sting_node);

        // get keys
        assert_eq!(get_nodes_or_keys("GET keys HTTPS/1.1".to_string(), nodes.lock().unwrap(), keys.lock().unwrap()), expected_sting_key);

        // bad request
        assert_eq!(get_nodes_or_keys("GET HTTPS/1.1".to_string(), nodes.lock().unwrap(), keys.lock().unwrap()), "GET request not in the right format.".to_string());
    }

    #[test]
    fn handle_client_test() {
        assert!(false);
    }
}