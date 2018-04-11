use std::sync::{Arc, Mutex};
use json::{OriginArray, Merged};

pub type OriginCache = Arc<Mutex<OriginArray>>;
pub type MergedCache = Arc<Mutex<Merged>>;
#[derive(Clone)]
pub enum Cache {
    Origin(OriginCache),
    Merged(MergedCache),
}
