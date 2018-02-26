static MERGED_FILE_PATH: &str = "target/merged.json";
// static ORIGIN_FILE_PATH: &str = "target/origin.json";
static LOCAL_TEST_URL: &str = "ws://localhost:3012";
static X_ACCESS_ID: &str = "X-Access-Id";
static X_SIGNATURE: &str = "X-Signature";
static X_NONCE: &str = "X-Nonce";

extern crate iron;
extern crate staticfile;
extern crate mount;
use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn http_server() {
    let mut mount = Mount::new();

    mount.mount("/merged/", Static::new(Path::new(MERGED_FILE_PATH)));

    Iron::new(mount).http("localhost:8080").unwrap();
}


use std::fs::OpenOptions;
use std::io::{
    BufWriter,
    Write
};

fn write_file(fname: &str, data: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(fname)?;
    let mut f = BufWriter::new(file);
    f.write_all(data.as_bytes())
}


extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


#[derive(Debug, Serialize, Deserialize)]
struct Bids {
    items: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Asks {
    items: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Depth {
    bids: Bids,
    asks: Asks,
    under: String,
    over: String,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Merged(Depth);

// #[derive(Debug, Serialize, Deserialize)]
// struct Origin {
//     origin: String,
//     snapshot: Depth,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct OriginArray(Vec<Origin>);

fn deserialize_merged(data: &str) -> Result<Merged, serde_json::Error> {
    serde_json::from_str(data)
}

// fn deserialize_origin(data: &str) -> Result<Origin, serde_json::Error> {
//     serde_json::from_str(data)
// }


extern crate sha2;
extern crate hmac;

use sha2::Sha256;
use hmac::{Hmac, Mac};

fn generate_x_signature(key: &str, data: &str) -> String {
    let mut hmac = Hmac::<Sha256>::new(key.as_bytes()).unwrap();
    hmac.input(data.as_bytes());
    let result = hmac.result();
    let code_bytes = result.code();
    let x_signature = code_bytes.map(|v| format!("{:02x}", v)).join("");
    x_signature.to_string()
}


extern crate ws;
extern crate url;

use ws::connect;
use std::thread;

struct Client;
impl ws::Handler for Client {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let json = &msg.as_text()?;
        if let Ok(json) = deserialize_merged(json) {
            write_file(MERGED_FILE_PATH, &serde_json::to_string(&json).unwrap())?;
        }
        Ok(())
    }

    fn build_request(&mut self, url: &url::Url) -> ws::Result<ws::Request> {
        let mut req = ws::Request::from_url(url)?;
        let key = "secret key";
        let data = "This is a pen.";
        let x_signature = generate_x_signature(key, data);
        let x_nonce = 18;
        let x_signature = x_signature + &format!("{:x}", x_nonce);
        {
            let headers = (&mut req).headers_mut();
            &headers.push((X_ACCESS_ID.to_string(), b"12".to_vec()));
            &headers.push((X_NONCE.to_string(), x_nonce.to_string().as_bytes().to_vec()));
            &headers.push((X_SIGNATURE.to_string(), x_signature.as_bytes().to_vec()));
        }

        Ok(req)
    }
}

fn websocket_client(url: &str) {
    connect(url, |_| {
        Client
    }).unwrap();
}

fn main() {
    let server = thread::spawn(move || {
        http_server();
    });

    let ws_origin = thread::spawn(move || {
        websocket_client(LOCAL_TEST_URL);
    });

    let _ = server.join();
    let _ = ws_origin.join();
}
