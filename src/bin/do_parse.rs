use std::{env::args, fs::File, io::BufReader};

use quick_xml::reader::Reader;
use rayon::prelude::*;
use wiki_xml::{page::Page, page_parser::PageParser};

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
    let reader = get_given_file_reader().unwrap();
    let parser = PageParser::new(Reader::from_reader(reader));

    let mut redirects: Vec<Page> = vec![];
    let mut root_pages: Vec<Page> = vec![];

    for page in parser {
        match page.redirect {
            Some(_) => redirects.push(page),
            None => root_pages.push(page),
        }
    }

    let redirects: Vec<(String, String)> = redirects
        .into_par_iter()
        .map(|page| (page.name, page.redirect.unwrap()))
        .collect();

    root_pages.iter_mut().for_each(|page| {
        page.tags = page
            .tags
            .par_iter()
            .map(|tag| {
                for (from, to) in redirects.iter() {
                    if from == tag {
                        println!("Matched, {from} : {to}");
                        return to.clone();
                    }
                }
                return tag.clone();
            })
            .collect();
    })
}
