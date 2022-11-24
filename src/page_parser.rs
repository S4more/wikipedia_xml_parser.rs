use std::{fs::File, io::BufReader};

use once_cell::sync::Lazy;
use quick_xml::{events::Event, Error, Reader};
use regex::Regex;

use crate::page::Page;

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\[\[).+?(]|\|)").unwrap());

static NAME_EXCLUDER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\(disambiguation\)|File:|.+:.+").unwrap());

pub struct PageParser {
    buf: Vec<u8>,
    reader: Reader<BufReader<File>>,
    write_size_interval: u64,
    file_index: u64,
    _done: bool,
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
                // file.write(json_string.as_bytes()).unwrap();
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
impl Iterator for PageParser {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self._done {
            return None;
        }

        self.to_next("title");
        let mut name: String = match self.next() {
            Ok(Event::Text(text)) => core::str::from_utf8(&text).unwrap_or("").to_string(),
            _ => "".to_string(),
        };

        // Remove disambiguation articles
        while NAME_EXCLUDER.is_match(name.as_str()) {
            self.to_next("title");
            name = match self.next() {
                Ok(Event::Text(text)) => core::str::from_utf8(&text).unwrap_or("").to_string(),
                _ => "".to_string(),
            };
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
                        break;
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

        return Some(Page::new(name, tags));
    }
}
