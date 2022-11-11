use std::fs::File;

use std::io::BufReader;
use std::{env::args, time::Instant};

use once_cell::sync::Lazy;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::Error;
use regex::Regex;

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[[^\[\]]+]]").unwrap());

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
    _done: bool,
}

#[derive(Debug)]
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
    pub fn new(reader: Reader<BufReader<File>>) -> Self {
        Self {
            buf: vec![],
            reader,
            _done: false,
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
                            tags.push(matches.as_str().to_string());
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
            println!("Page: {:} {:}", page.tags.len(), page.name);
            pages.push(page);
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

fn main() {
    let start = Instant::now();
    let reader = get_given_file_reader().expect("Failed to get file");

    let parser = Reader::from_reader(reader);
    let mut parser = PageParser::new(parser);

    let parsed = parser.parse();
    println!("{}", parsed.len());

    for page in parsed {
        println!("Page: {:} {:}", page.tags.len(), page.name);
    }

    // consume_parser(parser);

    let duration = start.elapsed();
    println!("Took : {duration:?}");
}
