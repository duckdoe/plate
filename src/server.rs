use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::request::parse_request;

pub struct Server {
    pub name: String,
}

impl Server {
    fn handle_client(&self, mut stream: TcpStream) {
        let mut s = String::new();
        let mut buffer = [0; 512];
        let data = stream.read(&mut buffer);

        match data {
            Ok(d) => {
                s.push_str(str::from_utf8(&buffer[..d]).expect("Unable to read from buffer."));
                let request = parse_request(s);

                match request {
                    Ok(req) => println!("{req:?}"),
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
