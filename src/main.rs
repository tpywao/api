extern crate iron;
extern crate ws;
extern crate url;
extern crate sha2;
extern crate hmac;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod http_server;
mod websocket_client;
mod file_io;
mod auth;
mod json;

use http_server::http_server;
use websocket_client::websocket_client;

use std::thread;
use json::Stream;

fn main() {
    let server = thread::spawn(move || {
        http_server(http_server_config::get_netloc(), file_config::get_origin_path(), file_config::get_merged_path());
    });

    let ws_origin = thread::spawn(move || {
        websocket_client(Stream::Origin, websocket_client_config::get_origin_url(), file_config::get_origin_path());
    });

    let ws_merged = thread::spawn(move || {
        websocket_client(Stream::Merged, websocket_client_config::get_merged_url(), file_config::get_merged_path());
    });

    let _ = server.join();
    let _ = ws_origin.join();
    let _ = ws_merged.join();
}

mod config {
    use std::env::var;

    pub fn get_env(key: &str) -> String {
        var(key).unwrap()
    }
}

mod file_config {
    use config::get_env;

    const ORIGIN_FILE_ENV_KEY: &str = "ORIGIN_FILE_PATH";
    const MERGED_FILE_ENV_KEY: &str = "MERGED_FILE_PATH";

    pub fn get_origin_path() -> String {
        get_env(ORIGIN_FILE_ENV_KEY)
    }

    pub fn get_merged_path() -> String {
        get_env(MERGED_FILE_ENV_KEY)
    }
}

mod http_server_config {
    use config::get_env;

    const NETLOC_ENV_KEY: &str = "HTTP_NETLOC";

    pub fn get_netloc() -> String {
        get_env(NETLOC_ENV_KEY)
    }
}

mod websocket_client_config {
    use config;

    const ORIGIN_URL_ENV_KEY: &str = "ORIGIN_URL";
    const MERGED_URL_ENV_KEY: &str = "MERGED_URL";

    pub fn get_origin_url() -> String {
        config::get_env(ORIGIN_URL_ENV_KEY)
    }

    pub fn get_merged_url() -> String {
        config::get_env(MERGED_URL_ENV_KEY)
    }
}
