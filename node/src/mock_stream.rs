use std::io::{Read, Write};

pub struct MockTcpStream {
    pub read_data: Vec<u8>,
    pub write_data: Vec<u8>,
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