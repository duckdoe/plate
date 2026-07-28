use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::request::parse_request;
use crate::response::Response;

pub struct Server {
    pub name: String,
}

impl Server {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut s = String::new();
        let mut buffer = [0; 512];
        // TODO: make sure all data is read from the client before sending a response.

        let data = stream.read(&mut buffer);

        match data {
            Ok(d) => {
                s.push_str(str::from_utf8(&buffer[..d]).expect("Unable to read from buffer."));
                let request = parse_request(s);

                match request {
                    Ok(req) => {
                        let body = "Hello world!";
                        let content_type = String::from("text/plain");
                        let status_code = 200;

                        let headers = HashMap::from([
                            (String::from("Content-Type"), content_type),
                            (String::from("Content-Length"), body.len().to_string()),
                            (String::from("Connection"), String::from("close")),
                        ]);

                        let response = Response::new(
                            Some(body.to_string()),
                            status_code,
                            headers,
                            req.version,
                        );
                        let res = response.to_string();
                        let res = stream.write_all(response.as_bytes());

                        match res {
                            Ok(_res) => {
                                println!("{} {} {}\n", req.method, req.path, response.status_code)
                            }
                            Err(_e) => println!("Failed to write a response"),
                        };
                    }
                    Err(_e) => {
                        println!("Some kind of error occured i can't explain but it happened.")
                    }
                }
            }
            Err(_e) => println!(),
        }
    }

    pub fn listen(&self, address: &str) {
        let listener = TcpListener::bind(address);

        match listener {
            Ok(l) => {
                println!("Connection running on: {address}");

                for stream in l.incoming() {
                    match stream {
                        Ok(s) => {
                            self.handle_client(s);
                        }
                        Err(e) => println!("Connection failed: {e}"),
                    }
                }
            }

            Err(e) => println!("Connection failed: {e}"),
        }
    }
}
