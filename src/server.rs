use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    handler::{Handler, Response, Router},
    request::HTTPMethod,
};
use crate::{request::parse_request, response::write_response};

pub struct Server {
    pub name: String,
    router: Router,
}

impl Server {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            router: Router::new(),
        }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut req = Vec::new();

        let mut s = String::new();
        let mut buffer = [0; 8192];

        // TODO: make sure all data is read from the client before sending a response.

        let mut data = stream.read(&mut buffer);
        let mut len = 0;

        match data {
            Ok(d) => {
                len += d;
                req.extend_from_slice(&buffer);
            }
            Err(ref e) => println!("Connection failed: {e}"),
        }

        while !str::from_utf8(&buffer[..len])
            .expect("Unable to read from buffer")
            .contains("\r\n\r\n")
        {
            data = stream.read(&mut buffer);

            match data {
                Ok(d) => {
                    len += d;
                    req.extend_from_slice(&buffer);
                }
                Err(ref e) => println!("Connection failed: {e}"),
            };
        }

        // Parse the headers and check if we need a body.

        let mut req_parts = str::from_utf8(&buffer[..len])
            .expect("Unable to read from buffer")
            .split("\r\n");

        let _line = req_parts.next().unwrap(); // request line
        let mut headers: HashMap<String, String> = HashMap::new();

        loop {
            let part = req_parts.next().unwrap();

            if part.is_empty() {
                break;
            }

            let header = part.split_once(":").unwrap();

            headers.insert(header.0.to_string(), header.1.to_string());
        }

        if let Some(value) = headers.get("Content-Length") {
            let content_length = value
                .trim()
                .parse::<usize>()
                .expect("Unable to parse content length");
            let data_len = len + content_length;

            while data_len > len {
                data = stream.read(&mut buffer);

                match data {
                    Ok(d) => {
                        len += d;
                        req.extend_from_slice(&buffer);
                    }
                    Err(ref e) => println!("Connection failed: {e}"),
                }
            }
        }

        match data {
            Ok(d) => {
                s.push_str(str::from_utf8(&req[..len]).expect("Unable to read from buffer."));
                let request = parse_request(s);

                match request {
                    Ok(req) => {
                        let content_type = String::from("text/plain");

                        let response = self.router.look_up(&req.method, req.path.as_str());

                        match response {
                            Ok(res) => {
                                let route_type = res.1;
                                let res = res.0(&req);

                                write_response(
                                    stream,
                                    req,
                                    res.body,
                                    res.status_code,
                                    content_type,
                                );
                            }

                            Err(_err) => {
                                write_response(
                                    stream,
                                    req,
                                    "Page not Found".to_string(),
                                    404,
                                    content_type,
                                );
                            }
                        }
                    }

                    Err(_e) => {
                        println!("Some kind of error occured i can't explain but it happened.")
                    }
                }
            }
            Err(err) => println!("Connection failed: {err}"),
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

    pub fn get(&mut self, path: &str, handler: Handler) {
        let map = HashMap::from([(path.to_string(), handler)]);
        self.router.routes.insert(HTTPMethod::GET, map);
    }
}
