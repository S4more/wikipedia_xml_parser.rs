use rustc_hash::FxHashMap;
use std::{
    env::args,
    fs::File,
    io::{BufReader, Write},
};

use quick_xml::Reader;
use wiki_xml::{page_id, page_parser::PageParser};

fn get_given_file_reader() -> Option<BufReader<File>> {
    let arg: Vec<String> = args().collect();
    if let Some(path) = arg.get(1) {
        if let Ok(file) = File::open(path) {
            return Some(BufReader::with_capacity(8096, file));
        }
    }
    None
}

pub fn main() {
    println!("Loading Ids");
    let mut page_ids: FxHashMap<String, usize> = page_id::get_page_to_id_hashmap();

    println!("Loading Pages");
    let page_links = page_id::get_id_to_page();

    println!("Loading Links");
    let mut page_links: Vec<Vec<usize>> = page_links.into_iter().map(|_| (vec![])).collect();

    let reader = get_given_file_reader().expect("Failed to get file");

    let parser = Reader::from_reader(reader);

    let parser = PageParser::new(parser, 500);

    println!("starting parsing");
    let mut total_pages: usize = 0;
    let max_pages = page_links.len();

    for page in parser {
        total_pages += 1;

        if total_pages % 2000 == 0 {
            println!(
                "{total_pages}/{max_pages} {}%",
                (total_pages as f64 / max_pages as f64) * 100.0
            );
        }

        if let None = page_ids.get(&page.name) {
            // Page has no references to it
            page_ids.insert(page.name.clone(), page_links.len());
            page_links.push(vec![]);
        }

        if let Some(id) = page_ids.get(&page.name) {
            for tag in page.tags {
                if let Some(tag_id) = page_ids.get(&tag) {
                    page_links[*id].push(*tag_id);
                }
            }
        }
    }

    let mut file = File::create("test_links.json").unwrap();
    println!("Started Serializing...");
    let json_string = serde_json::to_string(&page_links).unwrap();
    println!("Started writing...");
    file.write(json_string.as_bytes()).unwrap();
}
