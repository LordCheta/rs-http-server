use crate::http::Request;
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    address: String
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self) {

        let listener = TcpListener::bind(&self.address).unwrap();

        println!("Server started on: {}", self.address);

        loop {

            match listener.accept() {

                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    
                                },

                                Err(e) => println!("Failed to parse request with error: {}", e),
                            }
                        }

                        Err(e) => println!("Failed to read from connection with error: {}", e),
                    }
                },

                Err(e) => println!("Failed to connect with error: {}", e),

            }
        }
    }
}