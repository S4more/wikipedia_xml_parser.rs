use load_file;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Write};

pub fn main() {
    let bytes = load_file::load_bytes!(format!("../../parsed_all.json").as_str());
    let data: HashMap<String, u64> = serde_json::from_slice(bytes).unwrap();

    let mut pairs: Vec<(&String, &u64)> = data.iter().collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal));

    let mut total_refs: u64 = 0;

    for (_, refs) in pairs.iter() {
        total_refs += *refs;
    }

    let total_refs = total_refs;
    println!("Total Refs   :{}", total_refs);

    let top_10_percent = {
        let mut collector: Vec<&String> = vec![];

        let mut refs: u64 = 0;
        let mut iter = pairs.iter();
        while refs < (total_refs / 10) * 8 {
            if let Some((value, &count)) = iter.next() {
                refs += count;
                collector.push(value);
            } else {
                break;
            }
        }
        collector
    };

    let pairs: Vec<&String> = pairs.iter().map(|v| v.0).collect();

    println!("Total Titles: {}", pairs.len());
    println!("Top 10 Percent Titles: {}", top_10_percent.len());

    let json_string = serde_json::to_string(&pairs).unwrap();
    let json_string_top = serde_json::to_string(&top_10_percent).unwrap();
    let mut file = File::create("ordered_titles.json").unwrap();
    let mut file_top = File::create("ordered_titles_top.json").unwrap();

    println!("Started writing. Top");
    file_top.write(json_string_top.as_bytes()).unwrap();
    println!("Started writing. All");
    file.write(json_string.as_bytes()).unwrap();
}
