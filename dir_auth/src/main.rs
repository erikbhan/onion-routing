extern crate native_tls;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    // TODO: Create a better cert
    let mut file = File::open("foo.p12").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Identity::from_pkcs12(&pkcs12, "").unwrap();

    let acceptor = TlsAcceptor::new(pkcs12).unwrap();
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

    // TODO: Error handling
    fn handle_client(mut stream: TlsStream<TcpStream>) {
        loop {
            let mut buf = [0u8; 4096];
            let num_bytes_read = stream.read(&mut buf).unwrap();
            let data = std::str::from_utf8(&buf[0..num_bytes_read]).unwrap();
            println!("{}", data);
            stream.write_all(b"HTTP/1.1 200 OK").unwrap();
        }
    }

    // TODO: Graceful shutdown
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}