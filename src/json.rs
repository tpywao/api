use serde_json::{Value, Map};
pub use serde_json::{from_str, to_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bids {
    pub items: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asks {
    pub items: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    pub bids: Bids,
    pub asks: Asks,
    pub under: String,
    pub over: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Merged(pub Depth);
