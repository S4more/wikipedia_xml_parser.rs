use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub name: String,
    pub tags: Vec<String>,
    pub redirect: Option<String>,
}

impl Page {
    pub fn create(name: String, tags: Vec<String>, redirect: Option<String>) -> Self {
        Self {
            name,
            tags,
            redirect,
        }
    }

    pub fn new(name: String, tags: Vec<String>) -> Self {
        Self::create(name, tags, None)
    }

    pub fn redirect(name: String, redirect: String) -> Self {
        Self::create(name, vec![], Some(redirect))
    }
}
