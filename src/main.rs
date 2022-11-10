use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{env, thread};
use std::fs::{self, File};
use once_cell::sync::Lazy;
use regex::Regex;
use threadpool::ThreadPool;

static CONTENT: Lazy<String> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("File not found. Pass the file name with cargo run -- file.txt.");
    contents
});

static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| {
    let re = Regex::new(r"\[\[.+?(]|\|)").unwrap();
    re
});

static TITLE_MATCHER: Lazy<Regex> = Lazy::new(|| {
    let get_page_regex = Regex::new(r"<title>[\s\S]*?</title>").unwrap();
    get_page_regex
});

fn main() {
    let start = Instant::now();
    let n_workers = 32;
    let pool = ThreadPool::new(n_workers);

    // Divide the page into smaller sub-sections so we can multi thread it
    let get_page_regex = Arc::new(Regex::new(r"<page>[\s\S]*?</page>").unwrap());

    let it = get_page_regex.captures_iter(&CONTENT);
    let pages: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));


    for page in it {
        let clone = pages.clone();
        pool.execute(move || {
            let tags = parse_page(&page[0].to_string());
            clone.lock().unwrap().insert(get_page_title(&page[0].to_string()), tags);
        });
    }

    pool.join();

    let complete_page = pages.lock().unwrap();

    let mut stats = vec![];
    for page in complete_page.iter() {
        stats.push(format!("Page: {} - tags: {} ", page.0, page.1.len()));
    }
    //
    let mut file = File::create("out_parsed_two.txt").unwrap();
    let json_string = serde_json::to_string(&complete_page.to_owned()).unwrap();
    file.write(json_string.as_bytes()).unwrap();

    let mut stats_file = File::create("stats.txt").unwrap();
    stats_file.write(stats.join("\n").as_bytes()).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn get_page_title(content: &String) -> String {
    let page_tag_range = &TITLE_MATCHER.find(&content).unwrap();
    return content[page_tag_range.start() .. page_tag_range.end()].to_string();
}

fn parse_page(content: &String) -> Vec<String> {
    let mut matches: Vec<String> = vec![];
    let it = TAG_MATCHER.captures_iter(&content);

    for cap in it {
        let size = cap[0].len();
        let trimmed = &cap[0][2..size-1];
        if trimmed.starts_with("File: ") {
            continue;
        }

        matches.push(trimmed.to_string());
    }

    matches
}
