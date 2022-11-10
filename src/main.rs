use std::fs::File;

use std::io::BufReader;
use std::{env::args, time::Instant};

use once_cell::sync::Lazy;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex::Regex;

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[(.+?)]]|\|").unwrap());

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

fn consume_parser(mut parser: Reader<BufReader<File>>) {
    let mut buf = vec![];
    let page = String::from("page");
    let title = String::from("text");

    let mut is_page = false;
    let mut is_title = false;
    let mut titles: u64 = 0;

    loop {
        if buf.len() > 8096 {
            buf.clear()
        }

        match (parser.read_event_into(&mut buf), is_page, is_title) {
            (Ok(Event::Start(content)), false, false) => {
                match String::from_utf8(content.name().0.to_vec()).unwrap() == page {
                    true => is_page = true,
                    _ => {}
                }
            }

            (Ok(Event::Start(content)), true, false) => {
                match String::from_utf8(content.name().0.to_vec()).unwrap() == title {
                    true => is_title = true,
                    _ => {}
                }
            }

            (Ok(Event::End(content)), true, false) => {
                match String::from_utf8(content.name().0.to_vec()).unwrap() == page {
                    true => is_page = false,
                    _ => {}
                }
            }

            (Ok(Event::End(content)), true, true) => {
                match String::from_utf8(content.name().0.to_vec()).unwrap() == title {
                    true => is_title = false,
                    _ => {}
                }
            }

            (Ok(Event::Text(content)), true, true) => {
                if let Some(matches) = TAG_MATCHER.captures(core::str::from_utf8(&content).unwrap())
                {
                    if let Some(text) = matches.get(1) {
                        titles += 1;
                        if titles % 100 == 0 {
                            println!("{}", titles);
                        }
                        // println!("{}", text.as_str());
                    }
                }
            }

            (Ok(Event::Eof), _, _) => {
                println!("{:?}", buf.capacity());
                break;
            }
            _ => {}
        }
    }
}

fn main() {
    let start = Instant::now();
    let reader = get_given_file_reader().expect("Failed to get file");

    // let parser = quick_xml::Reader::
    let parser = Reader::from_reader(reader);

    consume_parser(parser);

    let duration = start.elapsed();
    println!("Took : {duration:?}");
}
