use std::{
    collections::HashMap,
    fs,
    io::Read,
    net::{TcpListener, TcpStream},
    path::Path,
};

use crate::{
    handler::{Handler, Request, Router},
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

        let mut data = stream.read(&mut buffer);

        match data {
            Ok(0) => {
                println!("Connection closed unexpectedly, client did not send anything");
                return;
            }

            Ok(d) => {
                req.extend_from_slice(&buffer[..d]);
            }
            Err(ref e) => println!("Connection failed: {e}"),
        }

        while !str::from_utf8(&req[..req.len()])
            .expect("Unable to read from buffer")
            .contains("\r\n\r\n")
        {
            data = stream.read(&mut buffer);

            match data {
                Ok(0) => break,
                Ok(d) => {
                    req.extend_from_slice(&buffer[..d]);
                }
                Err(ref e) => println!("Connection failed: {e}"),
            };
        }

        // Parse the headers and check if we need a body.
        let mut req_parts = str::from_utf8(&req[..req.len()])
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
            let data_len = req.len() + content_length;

            while data_len > req.len() {
                data = stream.read(&mut buffer);

                match data {
                    Ok(0) => break,
                    Ok(d) => {
                        req.extend_from_slice(&buffer[..d]);
                    }
                    Err(ref e) => println!("Connection failed: {e}"),
                }
            }
        }

        match data {
            Ok(_d) => {
                s.push_str(str::from_utf8(&req[..req.len()]).expect("Unable to read from buffer."));
                let request = parse_request(s);

                match request {
                    Ok(req_ok) => {
                        let req = Request::new(
                            req_ok.method,
                            req_ok.path,
                            req_ok.version,
                            req_ok.headers,
                            req_ok.body,
                        );
                        let content_type = String::from("text/plain");

                        let response = self.router.look_up(&req.method, req.path.as_str());

                        match response {
                            Ok(res) => {
                                let res = res(&req);

                                write_response(
                                    stream,
                                    req,
                                    res.body,
                                    res.status_code,
                                    content_type,
                                );
                            }

                            Err(_err) => {
                                // Fallback to files if no register api

                                let root = std::fs::canonicalize(&self.router.static_dir);

                                let root_str;
                                match root {
                                    Ok(req) => {
                                        root_str = req;
                                    }
                                    Err(_e) => {
                                        write_response(
                                            stream,
                                            req,
                                            "Not Found".to_string(),
                                            404,
                                            content_type,
                                        );

                                        return;
                                    }
                                };

                                let mut path = req.path.as_str();

                                if path == "/" {
                                    path = "index.html";
                                }

                                let requested = root_str.join(path.trim_start_matches('/'));
                                let requested = std::fs::canonicalize(requested);

                                let file_requested;

                                match requested {
                                    Ok(req) => {
                                        file_requested = req;
                                    }
                                    Err(_e) => {
                                        write_response(
                                            stream,
                                            req,
                                            "Not Found".to_string(),
                                            404,
                                            content_type,
                                        );

                                        return;
                                    }
                                };

                                if !file_requested.starts_with(&root_str) {
                                    // Reject it — attempted path traversal

                                    write_response(
                                        stream,
                                        req,
                                        "Permission Error".to_string(),
                                        403,
                                        content_type,
                                    );

                                    return;
                                }

                                let path = file_requested
                                    .to_str()
                                    .expect("Unable to parse into a string");

                                if !Path::new(&path).exists() {
                                    write_response(
                                        stream,
                                        req,
                                        "Not Found".to_string(),
                                        404,
                                        content_type,
                                    );
                                } else {
                                    let file =
                                        path.split_once(".").expect("Unable to split file").1;

                                    let content_type = match file {
                                        "html" => "text/html",
                                        "css" => "text/css",
                                        "js" => "text/javascript",
                                        "txt" => "text/plain",
                                        _ => "application/octet-stream",
                                    };

                                    let body =
                                        fs::read_to_string(path).expect("Could not read from file");

                                    write_response(
                                        stream,
                                        req,
                                        body,
                                        200,
                                        content_type.to_string(),
                                    );
                                }
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
