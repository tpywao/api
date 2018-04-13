use rustc_serialize::hex::ToHex;
use crypto::{
    digest::Digest,
    mac::Mac,
    hmac,
    sha2,
};
use url::Url;

pub fn generate_x_signature(key: &str, url: &Url, x_nonce: u64) -> String {
    let mut url = url.clone();
    url.set_port(None).unwrap();
    let host_path = url.as_str().trim_left_matches("ws://").trim_left_matches("wss://");

    let mut hasher = sha2::Sha256::new();
    hasher.input_str(host_path);
    let sha256_host_path = hasher.result_str();

    let to_sig = format!("{}{}", sha256_host_path, x_nonce);

    let mut hmac = hmac::Hmac::new(sha2::Sha256::new(), key.as_bytes());
    hmac.input(to_sig.as_bytes());
    hmac.result().code().to_hex()
}
