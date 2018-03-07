use ws::connect;
use ws::{
    Handler,
    Message,
    Result,
    Request
};
use url::Url;

use super::file_io::write_file;
use super::auth::generate_x_signature;
use super::json::{Stream, OriginArray, Merged};
use super::json::{
    to_string,
    from_str
};


struct Client {
    stream: Stream,
    file_path: String,
}
impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let text = &msg.as_text()?;
        match self.stream {
            Stream::Merged => {
                if let Ok(Merged(json)) = from_str(text) {
                    write_file(&self.file_path, &to_string(&json).unwrap())?;
                }
            }
            Stream::Origin => {
                if let Ok(OriginArray(json)) = from_str(text) {
                    write_file(&self.file_path, &to_string(&json).unwrap())?;
                }
            }
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
            &headers.push(("X-Access-Id".to_string(), b"12".to_vec()));
            &headers.push(("X-Nonce".to_string(), x_nonce.to_string().as_bytes().to_vec()));
            &headers.push(("X-Signature".to_string(), x_signature.as_bytes().to_vec()));
        }

        Ok(req)
    }
}

pub fn websocket_client(stream: Stream, url: String, file_path: String) {
    connect(url, |_| {
        Client {
            stream,
            file_path: file_path.to_string(),
        }
    }).unwrap();
}
