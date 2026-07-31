use crate::{handler::Response, request::Request, server::Server};

mod handler;
mod request;
mod response;
mod server;

fn greeting(request: &Request) -> Response {
    Response::new("Hello world", 200)
}

fn main() {
    let mut plate = Server::new("fortune");

    plate.get("/hello", greeting);
    plate.listen("127.0.0.1:8001");
}
