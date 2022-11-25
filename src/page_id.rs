use rustc_hash::FxHashMap;

pub fn get_id_to_page() -> Vec<String> {
    serde_json::from_str(load_file::load_str!("../ordered_titles.json")).unwrap()
}

pub fn get_page_to_id_hashmap() -> FxHashMap<String, usize> {
    get_id_to_page()
        .into_iter()
        .enumerate()
        .map(|val| (val.1, val.0))
        .collect::<FxHashMap<String, usize>>()
}

pub fn get_page_links() -> Vec<Vec<usize>> {
    print!("Loaded Links");
    let mut data: Vec<Vec<usize>> =
        serde_json::from_str(load_file::load_str!("../test_links.json")).unwrap();

    data.iter_mut().for_each(|item| item.sort());
    print!("Done Sorting");

    data
}
