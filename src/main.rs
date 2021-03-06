extern crate api;

use std::thread;

use api::{
    http_server::http_server,
    websocket_client::websocket_client,
    memory_cache::{
        OriginCache,
        MergedCache,
        Cache,
    },
};

fn main() {
    let origin_cache = OriginCache::default();
    let merged_cache = MergedCache::default();
    let ws_origin_cache = Cache::Origin(origin_cache.clone());
    let ws_merged_cache = Cache::Merged(merged_cache.clone());

    let server = thread::spawn(move || {
        http_server(
            http_server_config::get_netloc(),
            origin_cache.clone(),
            merged_cache.clone(),
            );
    });

    let ws_origin = thread::spawn(move || {
        websocket_client(
            websocket_client_config::get_origin_url(),
            websocket_client_config::get_api_key(),
            websocket_client_config::get_api_secret(),
            ws_origin_cache,
            websocket_client_config::get_ca_cert_path(),
            websocket_client_config::get_cert_path(),
            );
    });

    let ws_merged = thread::spawn(move || {
        websocket_client(
            websocket_client_config::get_merged_url(),
            websocket_client_config::get_api_key(),
            websocket_client_config::get_api_secret(),
            ws_merged_cache,
            websocket_client_config::get_ca_cert_path(),
            websocket_client_config::get_cert_path(),
            );
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

#[allow(dead_code)]
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
    use config::get_env;

    const ORIGIN_URL_ENV_KEY: &str = "ORIGIN_URL";
    const MERGED_URL_ENV_KEY: &str = "MERGED_URL";
    const API_KEY_ENV_KEY: &str = "API_KEY";
    const API_SECRET_ENV_KEY: &str = "API_SECRET";
    const CA_CERT_PATH_ENV_KEY: &str = "CA_CERT_PATH";
    const CERT_PATH_ENV_KEY: &str = "CERT_PATH";

    pub fn get_origin_url() -> String {
        get_env(ORIGIN_URL_ENV_KEY)
    }

    pub fn get_merged_url() -> String {
        get_env(MERGED_URL_ENV_KEY)
    }

    pub fn get_api_key() -> String {
        get_env(API_KEY_ENV_KEY)
    }

    pub fn get_api_secret() -> String {
        get_env(API_SECRET_ENV_KEY)
    }

    pub fn get_ca_cert_path() -> String {
        get_env(CA_CERT_PATH_ENV_KEY)
    }

    pub fn get_cert_path() -> String {
        get_env(CERT_PATH_ENV_KEY)
    }
}
