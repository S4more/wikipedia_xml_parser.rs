use rustc_hash::FxHashMap;

pub fn get_id_to_page() -> Vec<String> {
    let bytes = load_file::load_bytes!(format!("../ordered_titles.json").as_str());
    serde_json::from_slice(bytes).unwrap()
}

pub fn get_page_to_id_hashmap() -> FxHashMap<String, usize> {
    get_id_to_page()
        .into_iter()
        .enumerate()
        .map(|val| (val.1, val.0))
        .collect::<FxHashMap<String, usize>>()
}
