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

    for i in 1000..1100 {
        for j in 1000..1100 {
            if i == j {
                continue;
            }

            let start = Instant::now();
            let from = i;
            let to = j;

            println!("starting");
            let path = jason.find_path_root(from, to, 7);

            let duration = start.elapsed();
            total_elapsed = total_elapsed.add(duration);
            if duration > max_elapsed {
                max_path = path.clone();
            }
            max_elapsed = Duration::max(max_elapsed, duration);
            if let Some(path) = path {
                let path: Vec<(usize, &String)> =
                    path.iter().map(|id| (*id, jason.get_name(*id))).collect();
                println!("Took : {duration:?} for Path: {path:?}");
            } else {
                if jason.get_links(from).len() == 0 {
                    println!("No Outbound Links")
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
        let max_path: Vec<(&usize, &String)> = max_path
            .iter()
            .map(|id| (id, jason.get_name(*id)))
            .collect();
        println!("{max_path:?}");
    }

    println!("Total : {duration:?}");
    println!("WithoutPrint : {total_elapsed:?}");
    println!("Worst Case : {max_elapsed:?}");
}
