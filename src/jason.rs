use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// use crate::page_id::{get_id_to_page, get_page_links, get_page_to_id_hashmap};
use crate::page_id::*;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

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

    pub fn find_path_root(&self, from: usize, to: usize, ttl: usize) -> Option<Vec<usize>> {
        return self.find_path(from, to, ttl, Arc::new(AtomicBool::new(false)));
    }

    pub fn find_path(
        &self,
        from: usize,
        to: usize,
        ttl: usize,
        die: Arc<AtomicBool>,
    ) -> Option<Vec<usize>> {
        // let mut path: Vec<usize> = vec![];
        let connections = self.get_links(from);

        if connections.contains(&to) {
            die.store(true, Ordering::Release);
            return Some(vec![from, to]);
        }

        let die = Arc::clone(&die);
        for i in 1..ttl {
            if die.load(Ordering::Relaxed) {
                return None;
            }

            if let Some(res) = connections.par_iter().find_map_any(|conn| {
                if let Some(path) = self.find_path(*conn, to, i, die.clone()) {
                    let mut v = vec![from];
                    v.append(&mut path.clone());
                    return Some(v);
                }
                None
            }) {
                die.store(true, Ordering::Relaxed);
                return Some(res);
            }
        }
        None
    }
}
