use std::collections::HashMap;
use std::fmt;

#[derive(Hash, Eq, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
}

impl fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HTTPMethod::GET => write!(f, "GET"),
            HTTPMethod::POST => write!(f, "POST"),
        }
    }
}

pub enum HTTPVersion {
    HTTP11,
    HTTP10,
}

impl fmt::Display for HTTPVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HTTPVersion::HTTP11 => write!(f, "1.1"),
            HTTPVersion::HTTP10 => write!(f, "1.0"),
        }
    }
}

pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub version: HTTPVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub enum RequestError {
    HTTPMethodError,
    ParserError,
}

impl Request {
    pub fn new(
        method: HTTPMethod,
        path: String,
        version: HTTPVersion,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Self {
            method,
            path,
            version,
            headers,
            body,
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Request (\nmethod: {},\npath: {},\nversion: {}\nheaders: {:#?}\nbody: {:#?}\n)",
            self.method, self.path, self.version, self.headers, self.body
        )
    }
}

pub fn parse_request(request: String) -> Result<Request, RequestError> {
    let mut req_parts = request.split("\r\n");

    let line = req_parts.next().unwrap(); // request line
    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let part = req_parts.next().unwrap();

        if part.is_empty() {
            break;
        }

        let header = part.split_once(":").unwrap();

        headers.insert(header.0.to_string(), header.1.to_string());
    }

    let mut line_parts = line.split(" ");

    let method = line_parts.next().unwrap();
    let path = line_parts.next().unwrap().to_string();
    let mut version = line_parts.next().unwrap().split("/");

    let _http = version.next();
    let version = version.next().unwrap();

    match method {
        "GET" => {
            let version = if version == "1.1" {
                HTTPVersion::HTTP11
            } else {
                HTTPVersion::HTTP10
            };

            let mut body: Option<String> = None;

            if headers.contains_key("Content-Length") {
                let mut data = String::new();
                let content_length = headers
                    .get("Content-Length")
                    .expect("Content-Length does not exist");

                let content_length = content_length
                    .trim()
                    .parse::<usize>()
                    .expect("Unable to parse content length");

                while data.len() < content_length {
                    data.push_str(req_parts.next().unwrap());
                }

                body = Some(data);
            }

            Ok(Request::new(HTTPMethod::GET, path, version, headers, body))
        }
        "POST" => {
            let version = if version == "1.1" {
                HTTPVersion::HTTP11
            } else {
                HTTPVersion::HTTP10
            };

            let mut body: Option<String> = None;

            if headers.contains_key("Content-Length") {
                let mut data = String::new();
                let content_length = headers
                    .get("Content-Length")
                    .expect("Content-Length does not exist");

                let content_length = content_length
                    .trim()
                    .parse::<usize>()
                    .expect("Unable to parse content length");

                while data.len() < content_length {
                    data.push_str(req_parts.next().unwrap());
                }

                body = Some(data);
            }

            Ok(Request::new(HTTPMethod::POST, path, version, headers, body))
        }
        _ => Err(RequestError::HTTPMethodError),
    }
}
