use std::{collections::HashMap, error::Error};

use crate::request::{HTTPMethod, Request, RequestError};

pub struct Response {
    pub body: String,
    pub status_code: u16,
}

pub struct Reqeust {
    pub method: HTTPMethod,
    pub path: String,
    pub args: HashMap<String, String>, // for query strings
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
