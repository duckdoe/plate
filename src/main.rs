use crate::{handler::Request, handler::Response, server::Server};

mod handler;
mod request;
mod response;
mod server;

fn greeting(request: &Request) -> Response {
    let age = request.args.get("age"); // Will return a string so parse it bucko

    match age {
        Some(age) => {
            println!("{age}");
        }
        None => {
            println!("Is not a query string");
        }
    }

    Response::new("Hello world", 200)
}

fn main() {
    let mut plate = Server::new("fortune");

    plate.get("/hello", greeting);
    plate.listen("127.0.0.1:8001");
}
