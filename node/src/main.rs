extern crate native_tls;

use native_tls::{ TlsConnector };
use rand::{SeedableRng, RngCore};
use std::io::{Read, Write};
use std::net::{ TcpListener, TcpStream};
use rand::rngs::StdRng;

const DA_ADDR: &str = "0.0.0.0";
const DA_PORT: &str = "8443";
const PORT: &str = "3000";

fn get_key_and_send_to_da() -> [u8; 32] {
    let mut rng = StdRng::from_entropy();
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);

    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .build().expect("Error when building TLS connection");

    let stream = TcpStream::connect(format!("{}:{}", DA_ADDR, DA_PORT)).unwrap();

    // Domain will be ignored since cert/hostname verification is disabled
    let mut stream = connector.connect(DA_ADDR, stream).unwrap();

    stream.write_all(&key).unwrap();
    let mut res = vec![];

    // TODO: Error if not 200 OK
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));

    key
}

fn main() {
    let key = get_key_and_send_to_da();

    let listener = TcpListener::bind(format!("localhost:{}", PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    let num_bytes_read = stream.read(&mut buffer).unwrap();
    let data = std::str::from_utf8(&buffer[0..num_bytes_read]).unwrap();

    //For now, print received data
    println!("{}", data);

    // Answer incoming stream
    let res_ok = b"HTTP/1.1 200 OK\r\n";
    let num_read_bytes = stream.write(res_ok).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn handle_connection_test() {
        let input_bytes = b"Hello, from the testing stream!";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream);
        let mut buf = [0u8; 1024];
        stream.read_exact(&mut buf).unwrap();

        let expected_response = "HTTP/1.1 200 OK\r\n".to_string();
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn read(self: &mut MockTcpStream, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        let size: usize = std::cmp::min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Ok(size)
    }
}

impl Write for MockTcpStream {
    fn write(self: &mut MockTcpStream, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.write_data = Vec::from(buf);
        Ok(buf.len())
    }

    fn flush(self: &mut MockTcpStream) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}