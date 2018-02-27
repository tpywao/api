extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate ws;
extern crate url;
extern crate sha2;
extern crate hmac;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod config;
mod http_server;
mod websocket_client;
mod file_io;
mod auth;
mod json;

use http_server::http_server;
use websocket_client::websocket_client;

use std::thread;

use config::{
    LOCAL_HTTP_NETLOC,
    LOCAL_WS_URL
    // ORIGIN_STREAM_URL,
    // MERGED_STREAM_URL
};



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
