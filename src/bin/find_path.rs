use std::time::Instant;

use rand;
use wiki_xml::jason::Jason;
fn main() {
    let jason = Jason::new();

    println!("Finding Path");

    for _ in 0..2000 {
        let start = Instant::now();
        let from = rand::random::<u16>() as usize;
        let to = rand::random::<u16>() as usize;

        let path = jason.find_path(from, to, 8);

        let duration = start.elapsed();
        if let Some(path) = path {
            let path: Vec<&String> = path.iter().map(|id| jason.get_name(*id)).collect();
            println!("Path: {:?}", path);
        } else {
            println!(
                "Failed to find path: {} -> {}",
                jason.get_name(from),
                jason.get_name(to)
            );
        }
        println!("Took : {duration:?}");
    }
}
