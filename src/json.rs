use serde_json::{
    Value,
    Map,
};
pub use serde_json::{
    from_str,
    to_string,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bids {
    pub items: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Asks {
    pub items: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Depth {
    pub bids: Bids,
    pub asks: Asks,
    pub under: String,
    pub over: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Origin {
    pub origin: String,
    pub snapshot: Depth,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OriginArray(pub Vec<Origin>);

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Merged(pub Depth);
