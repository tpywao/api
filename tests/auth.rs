extern crate api;
extern crate url;

use api::auth::generate_x_signature;
use url::Url;

#[test]
fn test_generate_x_signature() {
    let api_secret = "API Secret";
    let url = Url::parse(
                "wss://localhost:443/?stream=origin"
                ).unwrap();
    let x_nonce = 100;
    assert_eq!(
        "cae76c11e37b8301174b34a6a99cc87b80aa4430a635d6e20b78158a87dd2732",
        generate_x_signature(
            api_secret,
            &url,
            x_nonce
            ));
}
