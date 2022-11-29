use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{env::args, time::Instant};

use once_cell::sync::Lazy;
use quick_xml::reader::Reader;
use regex::Regex;

use wiki_xml::page::Page;

use load_file;
use wiki_xml::page_parser::PageParser;

fn get_given_file_reader() -> Option<BufReader<File>> {
    let arg: Vec<String> = args().collect();
    if let Some(path) = arg.get(1) {
        if let Ok(file) = File::open(path) {
            return Some(BufReader::with_capacity(8096, file));
        }
    }
    None
}

// Somewhat off...
fn calculate_size(pages: &Vec<Page>) -> u64 {
    let mut total_size: u64 = 0;
    for page in pages.iter() {
        // 24 bytes is the size of a string in rust
        for tag in &page.tags {
            total_size += tag.len() as u64;
            // + 1 for each comma separated thing and + 2 for each quote
            total_size += "\"\",".len() as u64;
        }
        total_size += "\"tags\"".len() as u64;
        total_size += "\"name\": ".len() as u64;
        total_size += "[{}],".len() as u64
    }

    total_size
}

fn generate_count_files() {
    let mut map: HashMap<String, u64> = HashMap::new();
    for i in 0..253 {
        println!("File:{i}");
        let bytes = load_file::load_bytes!(format!("../parsed/parsed{i}.json").as_str());
        let data: Vec<Page> = serde_json::from_slice(bytes).unwrap();

        for page in data {
            for link in &page.tags {
                map.insert(link.to_string(), map.get(link).unwrap_or(&0) + 1);
            }
        }
    }
    let mut file = File::create("parsed_all.json").unwrap();
    let json_string = serde_json::to_string(&map).unwrap();
    println!("Started writing...");
    file.write(json_string.as_bytes()).unwrap();
}

fn main() {
    generate_count_files();

    return;

    // let start = Instant::now();
    // let reader = get_given_file_reader().expect("Failed to get file");

    // let parser = Reader::from_reader(reader);
    // let mut parser = PageParser::new(parser);

    // let parsed = parser.parse();

    // println!(
    //     "calculated size: {} mb",
    //     calculate_size(&parsed) / 1_048_576
    // );
    // println!("{}", parsed.len());

    // println!("Started serializing...");
    // let mut file = File::create("parsed/parsed.json").unwrap();
    // let json_string = serde_json::to_string(&parsed).unwrap();
    // println!("Started writing...");
    // file.write(json_string.as_bytes()).unwrap();

    // for page in parsed {
    //     println!("Page: {:} {:}", page.tags.len(), page.name);
    // }

    // // consume_parser(parser);

    // let duration = start.elapsed();
    // println!("Took : {duration:?}");
}
