use std::collections::HashMap;
use std::fs::File;

use std::io::{BufReader, Write};
use std::mem::{size_of, size_of_val};
use std::{env::args, time::Instant};

use once_cell::sync::Lazy;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::Error;
use regex::Regex;

use load_file;

use serde::{Deserialize, Serialize};

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\[\[).+?(]|\|)").unwrap());

static NAME_EXCLUDER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\(disambiguation\)|File:|.+:.+").unwrap());

// fn parse_content(content:)

fn get_given_file_reader() -> Option<BufReader<File>> {
    let arg: Vec<String> = args().collect();
    if let Some(path) = arg.get(1) {
        if let Ok(file) = File::open(path) {
            return Some(BufReader::with_capacity(8096, file));
        }
    }
    None
}

struct PageParser {
    buf: Vec<u8>,
    reader: Reader<BufReader<File>>,
    write_size_interval: u64,
    file_index: u64,
    _done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    name: String,
    tags: Vec<String>,
}

impl Page {
    pub fn new(name: String, tags: Vec<String>) -> Self {
        Self { name, tags }
    }
}

impl PageParser {
    pub fn new(reader: Reader<BufReader<File>>, write_size_interval: u64) -> Self {
        Self {
            buf: vec![],
            reader,
            _done: false,
            write_size_interval,
            file_index: 0,
        }
    }

    pub fn next(&mut self) -> Result<Event, Error> {
        if self.buf.len() > 8096 {
            self.clear_buffer();
        }
        self.reader.read_event_into(&mut self.buf)
    }

    pub fn clear_buffer(&mut self) {
        self.buf.clear();
    }

    pub fn done(&mut self) {
        self._done = true;
    }

    pub fn parse(&mut self) -> Vec<Page> {
        let mut pages: Vec<Page> = vec![];

        'outer: while !self._done {
            self.to_next("title");

            let name: String = match self.next() {
                Ok(Event::Text(text)) => core::str::from_utf8(&text).unwrap_or("").to_string(),
                _ => "".to_string(),
            };

            // Remove disambiguation articles
            if NAME_EXCLUDER.is_match(name.as_str()) {
                continue;
            }

            // println!("title: {name}");
            let mut tags: Vec<String> = vec![];
            loop {
                if self._done {
                    break;
                };
                match self.next() {
                    Ok(Event::End(tag)) => {
                        if core::str::from_utf8(tag.name().0).unwrap() == "page" {
                            break;
                        }
                    }
                    Ok(Event::Empty(tag)) => {
                        // Remove redirects from the list
                        if core::str::from_utf8(tag.local_name().into_inner())
                            .unwrap()
                            .contains("redirect")
                        {
                            continue 'outer;
                        }
                    }
                    Ok(Event::Text(text)) => {
                        let iter = TAG_MATCHER.find_iter(core::str::from_utf8(&text).unwrap_or(""));

                        for matches in iter {
                            let tag = matches.as_str();
                            if NAME_EXCLUDER.is_match(tag) {
                                continue;
                            }
                            tags.push(tag[2..tag.len() - 1].to_string());
                        }
                    }
                    Ok(Event::Eof) => {
                        println!("EOF Done");
                        self.done()
                    }
                    _ => {}
                }
            }

            let page = Page::new(name, tags);
            // println!("Page: {:} {:}", page.tags.len(), page.name);
            pages.push(page);

            if pages.len() % 25000 == 0 {
                let mut file =
                    File::create(format!("parsed/parsed{}.json", self.file_index)).unwrap();
                self.file_index += 1;
                let json_string = serde_json::to_string(&pages).unwrap();
                println!("Started writing.");
                file.write(json_string.as_bytes()).unwrap();
                println!("Wrote {} pages", self.file_index * 25000);
                pages.clear();
            }
        }

        return pages;
    }

    pub fn to_next(&mut self, name: &str) {
        loop {
            if self._done {
                return;
            };

            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::Start(tag)) => {
                    if core::str::from_utf8(tag.name().0).unwrap_or("") == name {
                        return;
                    }
                }
                Ok(Event::Eof) => self.done(),
                _ => {}
            }
        }
    }

    pub fn to_end(&mut self, name: &str, cb: impl Fn(Result<Event, quick_xml::Error>) -> ()) {
        loop {
            if self._done {
                return;
            };
            match self.reader.read_event_into(&mut self.buf) {
                Ok(Event::End(tag)) => {
                    if core::str::from_utf8(tag.name().0).unwrap_or("") == name {
                        break;
                    }
                }
                Ok(Event::Eof) => self.done(),
                e => cb(e),
            }
        }
    }
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
    // let mut parser = PageParser::new(parser, 20);

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

    // // for page in parsed {
    // //     println!("Page: {:} {:}", page.tags.len(), page.name);
    // // }

    // // consume_parser(parser);

    // let duration = start.elapsed();
    // println!("Took : {duration:?}");
}
