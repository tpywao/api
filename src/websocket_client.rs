use ws::connect;
use ws::{
    Handler,
    Message,
    Result,
    Request
};
use url::Url;

use super::config::{
    MERGED_FILE_PATH,
    X_ACCESS_ID,
    X_NONCE,
    X_SIGNATURE
};
use super::file_io::write_file;
use super::auth::generate_x_signature;
use super::json::Merged;
use super::json::{
    to_string,
    from_str
};


struct Client;
impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let text = &msg.as_text()?;
        if let Ok(Merged(json)) = from_str(text) {
            write_file(MERGED_FILE_PATH, &to_string(&json).unwrap())?;
        }
        Ok(())
    }

    fn build_request(&mut self, url: &Url) -> Result<Request> {
        let mut req = Request::from_url(url)?;
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

pub fn websocket_client(url: &str) {
    connect(url, |_| {
        Client
    }).unwrap();
}
