extern crate iron;
extern crate staticfile;
extern crate mount;
mod http_server;
use http_server::http_server;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
mod json;

extern crate sha2;
extern crate hmac;
mod auth;

extern crate ws;
extern crate url;
mod websocket_client;
use websocket_client::websocket_client;

use std::thread;

mod config;
use config::{
    LOCAL_HTTP_NETLOC,
    LOCAL_WS_URL
    // ORIGIN_STREAM_URL,
    // MERGED_STREAM_URL
};

mod file_io;


fn main() {
    let server = thread::spawn(move || {
        http_server(LOCAL_HTTP_NETLOC);
    });

    let ws_merged = thread::spawn(move || {
        websocket_client(LOCAL_WS_URL);
    });

    let _ = server.join();
    let _ = ws_merged.join();
}
