use crate::server::Server;

mod request;
mod response;
mod server;

fn main() {
    let plate = Server {
        name: String::from("fortune"),
    };

    plate.listen("127.0.0.1:8001");
}
