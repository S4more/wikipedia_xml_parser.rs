use load_file;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Write};

pub fn main() {
    let bytes = load_file::load_bytes!(format!("../../parsed_all.json").as_str());
    let data: HashMap<String, u64> = serde_json::from_slice(bytes).unwrap();

    let mut pairs: Vec<(&String, &u64)> = data.iter().collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal));
    let pairs: Vec<&String> = pairs.iter().map(|v| v.0).collect();
    println!("{}", pairs.len());

    let json_string = serde_json::to_string(&pairs).unwrap();
    let mut file = File::create("ordered_titles.json").unwrap();
    println!("Started writing.");
    file.write(json_string.as_bytes()).unwrap();
}
