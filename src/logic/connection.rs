use serde::de::DeserializeOwned;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

/// Primitive for reading JSON structure and writing string through TCP connection
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    /// Writes string through tcp stream
    pub fn write_string(&mut self, data: String) {
        let _ = self.stream.write_all(data.as_bytes());
    }

    /// Read and deserialize object from tcp stream
    pub fn read<T: DeserializeOwned>(&self) -> Option<T> {
        let mut buffer = vec![];
        return match BufReader::new(self.stream.try_clone().unwrap()).read_until(b'}', &mut buffer)
        {
            Err(error) => {
                println!("There is an error: {}", error);
                None
            }
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return None;
                }
                match serde_json::from_slice(&buffer) {
                    Ok(req) => Some(req),
                    Err(deserialize_error) => {
                        println!("{:#?}", String::from_utf8(buffer));
                        println!("JSON error: {}", deserialize_error);
                        None
                    }
                }
            }
        };
    }

    /// Returns ip and port of client
    pub fn name(&self) -> String {
        self.stream.peer_addr().unwrap().to_string()
    }
}
