use std::collections::HashMap;

use crate::request::{HTTPMethod, HTTPVersion};

pub struct Response {
    pub body: String,
    pub status_code: u16,
}

#[allow(dead_code)]
pub struct Request {
    pub method: HTTPMethod,
    pub path: String,
    pub version: HTTPVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub args: HashMap<String, String>, // for query strings
}

impl Request {
    pub fn new(
        method: HTTPMethod,
        path: String,
        version: HTTPVersion,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        let mut args = HashMap::new();
        let query_string = path.split_once("?");
        let queries;

        match query_string {
            Some(query) => {
                queries = query;
            }

            None => {
                return Self {
                    method,
                    path,
                    version,
                    headers,
                    body,
                    args,
                }
            }
        }

        let mut pairs = queries.1.split("&");

        while let Some(param) = pairs.next() {
            let mut parts = param.split('=');

            let key = parts.next().unwrap();
            let value = parts.next().unwrap();

            args.insert(key.to_string(), value.to_string());
        }

        Self {
            method,
            path: queries.0.to_string(),
            version,
            headers,
            body,
            args,
        }
    }
}

pub(crate) struct Router {
    pub(crate) static_dir: String,
    pub(crate) routes: HashMap<HTTPMethod, HashMap<String, Handler>>,
}

pub(crate) enum RouterError {
    NotFound,
}

pub type Handler = fn(&Request) -> Response;

impl Response {
    pub fn new(body: &str, status_code: u16) -> Self {
        Self {
            body: body.to_string(),
            status_code,
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Self {
            static_dir: String::from("./public"),
            routes: HashMap::new(),
        }
    }
    pub fn regiser(&mut self, method: HTTPMethod, path: &str, handler: Handler) {
        let handler = HashMap::from([(path.to_owned(), handler)]);

        self.routes.insert(method, handler);
    }

    pub fn look_up(&self, method: &HTTPMethod, path: &str) -> Result<Handler, RouterError> {
        let routes = self.routes.get(method);
        let handler;
        match routes {
            Some(r) => {
                let path_handler = r.get(path);

                handler = match path_handler {
                    Some(h) => Ok(*h), // Using a default for now.
                    None => Err(RouterError::NotFound),
                };
            }
            None => handler = Err(RouterError::NotFound),
        }

        handler
    }
}
