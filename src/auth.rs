use sha2::Sha256;
use hmac::{Hmac, Mac};

pub fn generate_x_signature(key: &str, data: &str) -> String {
    let mut hmac = Hmac::<Sha256>::new(key.as_bytes()).unwrap();
    hmac.input(data.as_bytes());
    let result = hmac.result();
    let code_bytes = result.code();
    let x_signature = code_bytes.map(|v| format!("{:02x}", v)).join("");
    x_signature.to_string()
}
