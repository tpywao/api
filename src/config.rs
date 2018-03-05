use std::env::var;

lazy_static! {
    pub static ref ORIGIN_FILE_PATH: String = var("ORIGIN_FILE_PATH").unwrap();
    pub static ref MERGED_FILE_PATH: String = var("MERGED_FILE_PATH").unwrap();
    // pub static ref ORIGIN_STREAM_URL: String = var("ORIGIN_STREAM_URL").unwrap();
    // pub static ref MERGED_STREAM_URL: String = var("MERGED_STREAM_URL").unwrap();
    pub static ref LOCAL_WS_URL: String = var("LOCAL_WS_URL").unwrap();
    pub static ref LOCAL_HTTP_NETLOC: String = var("LOCAL_HTTP_NETLOC").unwrap();
}
