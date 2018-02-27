use std::path::Path;
use iron::Iron;
use staticfile::Static;
use mount::Mount;

use super::config::MERGED_FILE_PATH;

pub fn http_server(netloc: &str) {
    let mut mount = Mount::new();

    mount.mount("/merged/", Static::new(Path::new(MERGED_FILE_PATH)));

    Iron::new(mount).http(netloc).unwrap();
}
