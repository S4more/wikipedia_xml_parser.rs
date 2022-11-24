use rustc_hash::FxHashMap;

use crate::page_id::{get_id_to_page, get_page_links, get_page_to_id_hashmap};

pub struct Jason {
    outbound_links: Vec<Vec<usize>>,
    page_names: Vec<String>,
    page_ids: FxHashMap<String, usize>,
}

impl Jason {
    // Creating a jason is very expensive
    // This will panic if the required files are not present
    pub fn new() -> Self {
        println!("Jason rises...");

        println!("Loading Id's ...");
        let page_names = get_id_to_page();
        println!("Loading Pages...");
        let page_ids = get_page_to_id_hashmap();
        println!("Loading Links...");
        let outbound_links = get_page_links();

        Self {
            page_names,
            page_ids,
            outbound_links,
        }
    }

    pub fn get_name(&self, id: usize) -> &String {
        &self.page_names[id]
    }

    pub fn get_id(&self, name: &String) -> Option<&usize> {
        self.page_ids.get(name)
    }

    pub fn get_links(&self, id: usize) -> &Vec<usize> {
        &self.outbound_links[id]
    }

    pub fn find_path(&self, from: usize, to: usize, ttl: usize) -> Option<Vec<usize>> {
        // let mut path: Vec<usize> = vec![];
        let connections = self.get_links(from);

        if connections.contains(&to) {
            return Some(vec![from, to]);
        } else {
            for i in 0..ttl {
                for &connection in connections {
                    if let Some(path) = self.find_path(connection, to, i) {
                        let mut v = vec![from];
                        v.append(&mut path.clone());
                        return Some(v);
                    }
                }
            }
        }
        None
    }
}
