use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};
use std::fs::{self, File};
use once_cell::sync::Lazy;
use regex::Regex;

static CONTENT: Lazy<Arc<String>> = Lazy::new(|| {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("File not found. Pass the file name with cargo run -- file.txt.");
    Arc::new(contents)
});

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // Divide the page into smaller sub-sections so we can multi thread it
    let get_page_regex = Arc::new(Regex::new(r"<page>[\s\S]*?</page>").unwrap());

    let it = get_page_regex.captures_iter(&CONTENT);
    let pages: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    println!("Starting threads.");

    for page in it {
        let clone = pages.clone();
        thread::spawn(move || {
            GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
            let tags = parse_page(&page[0].to_string());
            clone.lock().unwrap().insert(get_page_title(&page[0].to_string()), tags);
            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    }


    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        println!("Waiting... {} threads still running", GLOBAL_THREAD_COUNT.load(Ordering::SeqCst));
        thread::sleep(Duration::from_millis(1)); 
    }

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
}

fn get_page_title(content: &String) -> String {
    let get_page_regex = Regex::new(r"<title>[\s\S]*?</title>").unwrap();
    let page_tag = &get_page_regex.captures_iter(&content).into_iter().next().unwrap()[0];

    // println!("{}", page_tag);
    return page_tag.to_string();
}

fn parse_page(content: &String) -> Vec<String> {
    let re = Regex::new(r"\[\[.+?(]|\|)").unwrap();
    let mut matches: Vec<String> = vec![];
    let it = re.captures_iter(&content);

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
