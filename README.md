# Plate

A tiny HTTP server written from scratch in Rust.

Plate is a learning project: instead of reaching for `hyper` or `tokio`, it builds everything on top of the raw Rust standard library. The server binds a `TcpListener`, reads raw bytes off the socket, hand-parses the HTTP request, routes it, and hand-writes the response back. There are **zero external dependencies** — `Cargo.toml`'s dependency list is empty.

## Why Plate exists

I wanted to understand what actually happens between a browser sending a request and a server sending a response back. That means getting my hands dirty with the wire protocol itself — writing the request-line/header/body parser, building response status lines from scratch, and handling the fiddly edge cases (content lengths, path traversal, missing files) that a framework normally hides. Plate is the result of that exploration, and it's still very much a work in progress.

## Current features

- Hand-written HTTP/1.1 (and 1.0) request parsing: request line, headers, and body (via `Content-Length`)
- Routing for `GET`, `POST`, `DELETE`, and `PUT` with plain function handlers
- Query string parsing into `Request.args` (e.g. `/hello?age=25`)
- Static file serving from `./public` with content-type detection by extension (html, css, js, txt)
- Path traversal protection: requests that escape the static root get a `403`
- Hand-written table of ~60 HTTP status codes used to build response status lines
- Single-threaded, sequential connection handling
- Library + binary split, with `Server`, `Request`, and `Response` re-exported for external use

## Usage

```rust
use plate::{Request, Response, Server};

fn greeting(_request: &Request) -> Response {
    Response::new("Hello world", 200)
}

fn main() {
    let mut server = Server::new("plate");
    server.get("/hello", greeting);
    server.listen("127.0.0.1:8001");
}
```

Handlers are plain `fn(&Request) -> Response`. The `Request` exposes `method`, `path`, `version`, `headers`, `body`, and `args` (parsed query string).

## Installation / setup

Requires a recent Rust toolchain — the crate targets edition 2024, so Rust 1.85 or newer.

```sh
cargo build
```

## Running the example

The binary in `src/main.rs` registers a `/hello` route and starts the server on port `8001`:

```sh
cargo run
```

Then, in another terminal:

```sh
curl "http://127.0.0.1:8001/hello"        # -> "Hello world", 200
curl "http://127.0.0.1:8001/hello?age=25" # query strings land in request.args
curl "http://127.0.0.1:8001/"             # serves ./public/index.html
```

There are no automated tests yet.

## Project status

Early stage. Plate is a personal learning project and is **not production-ready**. It is single-threaded, closes every connection after a single response, and relies on a number of `unwrap`s — a malformed request can panic the server. Expect rough edges (and occasional unprintable commit messages).

## Roadmap

These aren't promises, just the obvious next steps visible in the code today:

- **Concurrency** — connections are currently handled one at a time in a loop
- **Exercise `POST`/`PUT`/`DELETE`** — route registration exists, but the example only uses `GET`
- **Connection keep-alive** — every response currently sets `Connection: close`
- **Better error handling** — replace panics/`unwrap`s on malformed input with proper `4xx` responses
- **Tests** — none exist yet
- **Use the server `name`** — it's stored but never sent anywhere
