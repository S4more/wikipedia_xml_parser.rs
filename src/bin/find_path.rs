use std::{
    ops::Add,
    time::{Duration, Instant},
};

use wiki_xml::jason::Jason;

fn main() {
    let jason = Jason::new();

    println!("Finding Path");

    let root_start = Instant::now();
    let mut total_elapsed: Duration = Duration::from_micros(0);
    let mut max_elapsed: Duration = Duration::from_micros(0);
    let mut max_path: Option<Vec<usize>> = None;

    for i in 0..100 {
        for j in 0..100 {
            if i == j {
                continue;
            }

            let start = Instant::now();
            let from = i;
            let to = j;

            let path = jason.find_path(from, to, 7);

            let duration = start.elapsed();
            total_elapsed = total_elapsed.add(duration);
            if duration > max_elapsed {
                max_path = path.clone();
            }
            max_elapsed = Duration::max(max_elapsed, duration);
            if let Some(path) = path {
                let path: Vec<&String> = path.iter().map(|id| jason.get_name(*id)).collect();
                println!("Path: {:?}", path);
                println!("Took : {duration:?}");
            } else {
                if jason.get_links(from).len() == 0 {
                } else {
                    println!(
                        "Failed to find path: {} -> {}",
                        jason.get_name(from),
                        jason.get_name(to)
                    );
                }
            }
        }
    }

    let duration = root_start.elapsed();
    if let Some(max_path) = max_path {
        let max_path: Vec<&String> = max_path.iter().map(|id| jason.get_name(*id)).collect();
        println!("{max_path:?}");
    }

    println!("Total : {duration:?}");
    println!("WithoutPrint : {total_elapsed:?}");
    println!("Worst Case : {max_elapsed:?}");
}
