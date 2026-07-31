use crate::request::{HTTPVersion, Request};
use core::fmt;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub(crate) struct Response {
    pub(crate) version: HTTPVersion,
    pub(crate) status_code: u16,
    pub(crate) body: Option<String>,
    pub(crate) headers: HashMap<String, String>,
}

impl Response {
    pub fn new(
        body: Option<String>,
        status_code: u16,
        headers: HashMap<String, String>,
        version: HTTPVersion,
    ) -> Self {
        Self {
            version,
            status_code,
            body,
            headers,
        }
    }
}

pub fn http_status_codes() -> HashMap<u16, &'static str> {
    HashMap::from([
        // 1xx Informational
        (100, "Continue"),
        (101, "Switching Protocols"),
        (102, "Processing"),
        (103, "Early Hints"),
        // 2xx Success
        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non-Authoritative Information"),
        (204, "No Content"),
        (205, "Reset Content"),
        (206, "Partial Content"),
        (207, "Multi-Status"),
        (208, "Already Reported"),
        (226, "IM Used"),
        // 3xx Redirection
        (300, "Multiple Choices"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (304, "Not Modified"),
        (305, "Use Proxy"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),
        // 4xx Client Errors
        (400, "Bad Request"),
        (401, "Unauthorized"),
        (402, "Payment Required"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (406, "Not Acceptable"),
        (407, "Proxy Authentication Required"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (410, "Gone"),
        (411, "Length Required"),
        (412, "Precondition Failed"),
        (413, "Content Too Large"),
        (414, "URI Too Long"),
        (415, "Unsupported Media Type"),
        (416, "Range Not Satisfiable"),
        (417, "Expectation Failed"),
        (418, "I'm a teapot"),
        (421, "Misdirected Request"),
        (422, "Unprocessable Content"),
        (423, "Locked"),
        (424, "Failed Dependency"),
        (425, "Too Early"),
        (426, "Upgrade Required"),
        (428, "Precondition Required"),
        (429, "Too Many Requests"),
        (431, "Request Header Fields Too Large"),
        (451, "Unavailable For Legal Reasons"),
        // 5xx Server Errors
        (500, "Internal Server Error"),
        (501, "Not Implemented"),
        (502, "Bad Gateway"),
        (503, "Service Unavailable"),
        (504, "Gateway Timeout"),
        (505, "HTTP Version Not Supported"),
        (506, "Variant Also Negotiates"),
        (507, "Insufficient Storage"),
        (508, "Loop Detected"),
        (510, "Not Extended"),
        (511, "Network Authentication Required"),
    ])
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Why tf did i write like this?

        let status_messages = http_status_codes();
        let status_message = *status_messages
            .get(&self.status_code)
            .unwrap_or(&"Unknown Status");

        let mut response_str = format!(
            "HTTP/{} {} {}\r\n",
            self.version, self.status_code, status_message
        )
        .to_string();
        let headers = &self.headers;

        for (k, v) in headers {
            let header = format!("{}: {}\r\n", k, v);
            response_str.push_str(header.as_str());
        }

        let body = &self.body;
        let body = body.as_ref().unwrap();

        response_str.push_str(format!("\r\n{}\r\n", body).as_str());

        write!(f, "{}", response_str)
    }
}

pub fn write_response(
    mut stream: TcpStream,
    req: Request,
    body: String,
    status_code: u16,
    content_type: String,
) {
    let headers = HashMap::from([
        (String::from("Content-Type"), content_type),
        (String::from("Content-Length"), body.len().to_string()),
        (String::from("Connection"), String::from("close")),
    ]);

    let response = Response::new(Some(body.to_string()), status_code, headers, req.version);
    let res = response.to_string();
    let res = stream.write_all(res.as_bytes());

    match res {
        Ok(_res) => {
            println!("{} {} {}\n", req.method, req.path, response.status_code)
        }
        Err(_e) => println!("Failed to write a response"),
    };
}
