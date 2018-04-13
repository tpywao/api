use ws::{
    connect,
    Handler,
    Message,
    Result,
    Request,
    util::TcpStream,
};
use url::Url;
use openssl::{
    ssl::{
        SslConnectorBuilder,
        SslMethod,
        SslStream,
    },
    x509,
};

use auth::generate_x_signature;
use json::from_str;
use memory_cache::Cache;

struct Client {
    api_key: String,
    api_secret: String,
    cache: Cache,
    ca_path: String,
    cert_path: String,
}
impl Handler for Client {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let text = &msg.as_text()?;
        if let Ok(json) = from_str(text) {
            if let Cache::Origin(cache) = self.cache.clone() {
                *cache.lock().unwrap() = json;
            }
        }
        if let Ok(json) = from_str(text) {
            if let Cache::Merged(cache) = self.cache.clone() {
                *cache.lock().unwrap() = json;
            }
        }
        Ok(())
    }

    fn upgrade_ssl_client(&mut self, sock: TcpStream, url: &Url) -> Result<SslStream<TcpStream>> {
        let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
        builder.set_ca_file(&self.ca_path).unwrap();
        builder.set_certificate_file(&self.cert_path, x509::X509_FILETYPE_PEM).unwrap();
        let connector = builder.build();
        connector.connect(url.domain().unwrap(), sock).map_err(From::from)
    }

    fn build_request(&mut self, url: &Url) -> Result<Request> {
        let mut req = Request::from_url(url)?;
        let x_nonce = 18;
        let x_signature = generate_x_signature(&self.api_secret, url, x_nonce);
        {
            let ref mut headers = req.headers_mut();
            headers.push(("X-Access-Id".to_owned(), self.api_key.as_bytes().to_vec()));
            headers.push(("X-Nonce".to_owned(), x_nonce.to_string().as_bytes().to_vec()));
            headers.push(("X-Signature".to_owned(), x_signature.as_bytes().to_vec()));
        }

        Ok(req)
    }
}

pub fn websocket_client(
    url: String,
    api_key: String, api_secret: String,
    cache: Cache,
    ca_path: String, cert_path: String) {
    connect(url, |_| {
        Client {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            cache: cache.to_owned(),
            ca_path: ca_path.to_owned(),
            cert_path: cert_path.to_owned(),
        }
    }).unwrap();
}
