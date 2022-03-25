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

fn handle_connection(mut stream: TcpStream) {
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
mod tests {
    // TODO: handle_connection
    // TODO: get_key_and_send_to_da
}