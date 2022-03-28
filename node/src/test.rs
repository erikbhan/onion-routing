use super::*;

#[test]
fn handle_connection_test() {
    let input_bytes = b"Hello, from the testing stream!";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = mock_stream::MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream);
    let mut buf = [0u8; 1024];
    stream.read_exact(&mut buf).unwrap();

    let expected_response = "HTTP/1.1 200 OK\r\n".to_string();
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}