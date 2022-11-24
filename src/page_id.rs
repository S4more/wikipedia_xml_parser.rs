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

pub fn get_page_links() -> Vec<Vec<usize>> {
    let bytes = load_file::load_bytes!(format!("../test_links.json").as_str());
    print!("Loaded Links");
    let mut data: Vec<Vec<usize>> = serde_json::from_slice(bytes).unwrap();

    data.iter_mut().for_each(|item| item.sort());
    print!("Done Sorting");

    data
}
