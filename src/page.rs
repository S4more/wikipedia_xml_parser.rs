use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub name: String,
    pub tags: Vec<String>,
}

impl Page {
    pub fn new(name: String, tags: Vec<String>) -> Self {
        Self { name, tags }
    }
}
